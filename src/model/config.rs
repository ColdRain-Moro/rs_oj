use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub server: Server,
    pub problems: Vec<Problem>,
    pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub bind_address: String,
    pub bind_port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Problem {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub cases: Vec<Case>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Case {
    pub score: f32,
    pub input_file: String,
    pub answer_file: String,
    pub time_limit: u64,
    pub memory_limit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub file_name: String,
    pub command: Vec<String>,
}
