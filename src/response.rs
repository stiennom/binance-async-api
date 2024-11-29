use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug, Clone)]
pub struct Response<T> {
    pub status: StatusCode,
    pub headers: Box<HeaderMap>,
    pub content: T,
}
