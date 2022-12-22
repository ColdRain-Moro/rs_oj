use serde::{Serialize, Deserialize};

use super::{RunResult, State};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostJobParams {
    pub source_code: String,
    pub language: String,
    pub problem_id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryJobParams {
    problem_id: Option<u32>,
    language: Option<String>,
    from: Option<String>,
    to: Option<String>,
    state: Option<State>,
    result: Option<RunResult>
}