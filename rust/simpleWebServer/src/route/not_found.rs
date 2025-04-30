use crate::http::constants;

pub fn get_response(_: &Vec<String>) -> String{
    return constants::HTTP_NOT_FOUND_HEADER.to_string();
}