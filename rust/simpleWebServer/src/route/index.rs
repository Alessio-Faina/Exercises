use crate::http;

pub fn get_response(http_request: &Vec<String>) -> String{
    let mut response : String = "".to_string();

    match http::http_parser::parse_for_agent(&http_request) {
        Ok(v) => {response += &v},
        Err(e) => {response += &e}
    }

    "".to_string()
}