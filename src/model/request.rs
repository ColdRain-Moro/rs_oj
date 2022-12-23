use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::model::Job;

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
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    state: Option<State>,
    result: Option<RunResult>
}

impl QueryJobParams {
    pub fn matches(&self, job: &Job) -> bool {
        Self::matches_one(&self.problem_id, &job.problem.id) &&
            Self::matches_one(&self.language, &job.language.name) &&
            Self::matches_one(&self.state, &job.state) &&
            Self::matches_one(&self.result, &job.result) &&
            self.matches_time(job)
    }
    
    fn matches_time(&self, job: &Job) -> bool {
        if let Some(from) = self.from { 
            if from > job.created_time { return false }
        }
        if let Some(to) = self.from {
            if to < job.created_time { return false }
        }
        true
    }

    fn matches_one<T: PartialEq>(option: &Option<T>, value: &T) -> bool {
        if let Some(v) = option {
            return v == value
        }
        true
    }
}