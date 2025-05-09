#[derive (Debug, Eq, Clone, Copy, Hash)]
pub enum HttpResponseStatus {
    Ok = 200,
    NotFound = 404
}

impl PartialEq for HttpResponseStatus {
    fn eq(&self, other: &Self) -> bool {
        if *self as u16 == *other as u16 { return true;}
        return false;
    }
}

pub static HTTP_SUCCESS_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
pub static HTTP_NOT_FOUND_HEADER: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub static HTTP_INTERNAL_ERROR : &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";