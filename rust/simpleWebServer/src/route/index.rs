use crate::http::constants;
use crate::http::http_parser;

pub fn get_response(http_request: &Vec<String>) -> String{
    let mut response : String = constants::HTTP_SUCCESS_HEADER.to_string();

    match http_parser::parse_for_agent(&http_request) {
        Ok(v) => {response += &v},
        Err(e) => {response += &e}
    }

    response += " user !\r\n";
    response += "You requested route ";
    response += &http_parser::get_route(&http_request);
    response += "\r\n";

    let a = http_parser::get_request_parameters(&http_request);
    for item in a {
        response += &item;
        response += "\r\n";
    }

    return response;
}