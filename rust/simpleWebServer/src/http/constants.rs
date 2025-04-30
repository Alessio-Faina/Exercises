#[derive (Debug, PartialEq)]
pub enum HttpResponseStatus {
    Ok = 200,
    NotFound = 404
}

pub static HTTP_SUCCESS_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
pub static HTTP_NOT_FOUND_HEADER: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";