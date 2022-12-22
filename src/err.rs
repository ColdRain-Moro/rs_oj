use std::error::Error;
use std::fmt;
use std::fmt::Display;
use actix_web::{ResponseError};

#[derive(Debug)]
pub struct AppError {
    message: String
}

impl AppError {
    pub fn new(message: String) -> Box<Self> {
        Box::new(Self { message })
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "异常信息: {}", self.message)
    }
}

impl ResponseError for AppError {}

impl Error for AppError {}