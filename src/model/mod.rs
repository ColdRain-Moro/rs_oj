use std::cell::{Cell, RefCell};
use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use wait_timeout::ChildExt;
use crate::ATOMIC_ID;
use crate::model::config::{Language, Problem};
use crate::model::request::PostJobParams;

pub mod config;
pub mod request;
pub mod response;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Job {
    id: u32,
    params: PostJobParams,
    language: Language,
    problem: Problem,
    state: State,
    created_time: DateTime<Utc>,
    updated_time: DateTime<Utc>,
    result: RunResult,
    cases: Vec<Case>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Case {
    id: u32,
    result: RunResult,
    time: u64,
    memory: u64,
    info: String,
}

impl Job {
    pub fn new(params: PostJobParams, problem: Problem, language: Language) -> Job {
        let lock = ATOMIC_ID.lock().unwrap();
        let id = lock.get();
        lock.set(id + 1);
        return Job {
            id, params, language, problem,
            state: State::Queueing,
            created_time: Utc::now(),
            updated_time: Utc::now(),
            result: RunResult::Waiting,
            cases: Vec::new()
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // 转绝对路径
        let source = self.source_path();
        // 父目录不存在则创建
        let source_dir_path = Path::new(&source).parent().unwrap();
        if !source_dir_path.exists() {
            fs::create_dir_all(source_dir_path)?;
        }
        // 创建文件
        let mut source_file = File::create(&source)?;
        // 代码写入进去
        source_file.write(self.params.source_code.as_bytes())?;
        let output = self.output_path();
        let output_dir_path = Path::new(&output).parent().unwrap();
        if !output_dir_path.exists() {
            fs::create_dir_all(output_dir_path)?
        }

        // stdout将要重定向到的临时文件
        let input = self.input_path();
        let input_dir_path = Path::new(&input).parent().unwrap();
        if !input_dir_path.exists() {
            fs::create_dir_all(input_dir_path)?
        }
        File::create(&input)?;

        // 开始编译，更新状态
        self.result = RunResult::Running;
        self.state = State::Running;
        let compile = |output: &str, input: &str| -> io::Result<RunResult> {
            let cmd: Vec<String> = self.language.command
                .iter()
                .map(|cmd| {
                    cmd.replace("%OUTPUT%", output)
                        .replace("%INPUT%", input)
                }).collect();
            // 起一个进程编译
            let mut process = Command::new(&cmd[0])
                .args(&cmd[1..])
                .spawn()?;
            let exitstatus = process.wait()?;
            if !exitstatus.success() { return Ok(RunResult::CompilationError); }
            Ok(RunResult::CompilationSuccess)
        };
        // 更新状态为编译结果
        self.result = compile(&output, &source)?;
        // 开始评测
        let mut run_case = |case: &config::Case, mut case_result: Case| -> io::Result<()> {
            let mut process = Command::new(&output)
                .stdin(Stdio::from(File::open(&case.input_file)?))
                .stdout(Stdio::from(File::create(&input)?))
                .stderr(Stdio::null())
                .spawn()?;
            let res = process.wait_timeout(Duration::from_micros(case.time_limit))?;
            // 超时未退出
            if let None = res {
                // 杀进程，释放内存空间和cpu算力
                process.kill()?;
                case_result.result = RunResult::TimeLimitExceeded;
                return Ok(())
            }
            // 正常退出，比对input和ans
            if fs::read_to_string(&input)? == fs::read_to_string(&case.answer_file)? {
                case_result.result = RunResult::Accepted;
            } else {
                case_result.result = RunResult::WrongAnswer;
            }
            // 放入cases
            self.cases.push(case_result);
            Ok(())
        };
        for (index, case) in self.problem.cases.iter().enumerate() {
            run_case(case, Case::new(index as u32))?;
        }
        if self.cases.iter().all(|case| case.result == RunResult::Accepted) {
            self.result = RunResult::Accepted;
        } else {
            self.result = RunResult::WrongAnswer;
        }
        self.state = State::Finished;
        self.clear_temp_files()?;
        Ok(())
    }

    pub fn system_error(&mut self) {
        self.state = State::Canceled;
        self.result = RunResult::SystemError;
    }

    // 清理临时文件
    fn clear_temp_files(&self) -> io::Result<()> {
        fs::remove_file(self.input_path())?;
        fs::remove_file(self.output_path())?;
        fs::remove_file(self.source_path())?;
        Ok(())
    }

    fn source_path(&self) -> String {
        format!("./problem/{}/source/{}.rs", self.problem.id, self.id)
    }

    fn output_path(&self) -> String {
        format!("./problem/{}/output/{}", self.problem.id, self.id)
    }

    fn input_path(&self) -> String {
        format!("./problem/{}/input/{}.txt", self.problem.id, self.id)
    }
}

impl Case {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            result: RunResult::Waiting,
            time: 0,
            memory: 0,
            info: "".to_string()
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum State {
    Queueing,
    Running,
    Finished,
    Canceled
}

// job result
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum RunResult {
    Waiting,
    Running,
    Accepted,
    #[serde(rename(serialize = "Compilation Error", deserialize = "Compilation Error"))]
    CompilationError,
    #[serde(rename(serialize = "Compilation Success", deserialize = "Compilation Success"))]
    CompilationSuccess,
    #[serde(rename(serialize = "Wrong Answer", deserialize = "Wrong Answer"))]
    WrongAnswer,
    #[serde(rename(serialize = "Runtime Error", deserialize = "Runtime Error"))]
    RuntimeError,
    #[serde(rename(serialize = "Time Limit Exceeded", deserialize = "Time Limit Exceeded"))]
    TimeLimitExceeded,
    #[serde(rename(serialize = "Memory Limit Exceeded", deserialize = "Memory Limit Exceeded"))]
    MemoryLimitExceeded,
    #[serde(rename(serialize = "System Error", deserialize = "System Error"))]
    SystemError,
    #[serde(rename(serialize = "SPJ Error", deserialize = "SPJ Error"))]
    SpjError,
    Skipped
}