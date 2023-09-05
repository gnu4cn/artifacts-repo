use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub message: String,
    pub data: Vec<T>,
    pub page_num: i32,
    pub page_size: i32,
    pub total_elements: i32,
}

impl<T> Page<T> {
    pub fn new(
        message: &str,
        data: Vec<T>,
        page_num: i32,
        page_size: i32,
        total_elements: i32,
    ) -> Page<T> {
        Page {
            message: message.to_string(),
            data,
            page_num,
            page_size,
            total_elements,
        }
    }
}
