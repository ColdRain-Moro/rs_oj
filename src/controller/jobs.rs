use std::sync::Mutex;

use actix_web::{post, Responder, web, get, put, delete, HttpResponse};

use crate::{AppState, model::request::{PostJobParams, QueryJobParams}};
use crate::err::AppError;
use crate::model::config::{Language, Problem};
use crate::model::Job;
use crate::model::response::BaseResponse;

#[post("/jobs")]
pub async fn post_job(body: web::Json<PostJobParams>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder, AppError> {
    let params = body.into_inner();
    let data = data.into_inner();
    // 先找到对应的问题
    let mut the_problem: Option<Problem> = None;
    let mut the_language: Option<Language> = None;
    // 这里拿了锁 下面编译执行代码会花很长时间
    // 如果到那里都不释放容易死锁 让他用完就释放
    {
        let config = &data.lock().unwrap().config;

        for problem in &config.problems {
            if problem.id == params.problem_id {
                the_problem = Some(problem.clone());
                break
            }
        }

        for language in &config.languages {
            if language.name == params.language {
                the_language = Some(language.clone());
                break
            }
        }
    }
    return if let Some(problem) = the_problem && let Some(language) = the_language {
        let mut job = Job::new(params, problem, language);
        // 如果执行出错则需要额外修改一下job的状态
        if let Err(_) = job.run() {
            job.system_error();
        }
        Ok(
            HttpResponse::Ok()
                .json(
                    BaseResponse::ok(job)
                )
        )
    } else {
        Ok(
            HttpResponse::Ok()
                .json(
                    // 呕 这可真是个败笔
                    BaseResponse::<Job>::bad_request("the problem does not exist!")
                )
        )
    }
}

#[get("/jobs")]
pub async fn get_jobs(query: web::Query<QueryJobParams>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder, AppError> {
    Ok("")
}

#[get("/jobs/{jobId}")]
pub async fn get_job_by_id(job_id: web::Path<i32>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder, AppError> {
    Ok("")
}

#[put("/jobs/{jobId}")]
pub async fn put_job_by_id(job_id: web::Path<i32>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder, AppError> {
    Ok("")
}

#[delete("/jobs/{jobId}")]
pub async fn delete_job_by_id(job_id: web::Path<i32>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder, AppError> {
    Ok("")
}