use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BaseResponse<T> {
    error_code: i16,
    message: String,
    data: Option<T>
}

impl<T> BaseResponse<T> {
    pub fn create(error_code: i16, message: &str, data: T) -> Self {
        Self { error_code, message: message.to_string(), data: Some(data) }
    }
    
    pub fn bad_request(message: &str) -> Self {
        Self { error_code: 400, message: message.to_string(), data: None }
    }

    pub fn ok(data: T) -> Self {
        Self { error_code: 0, message: "ok".to_string(), data: Some(data) }
    }
}
