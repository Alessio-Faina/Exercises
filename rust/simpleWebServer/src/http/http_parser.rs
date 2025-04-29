use super::constants;


pub fn parse_for_agent(http_request: &Vec<String>) -> Result<String, &'static str> {
    for item in http_request {
        if item.contains("User-Agent") {
            let l : Vec<&str> = item.split(" ").collect();
            return Ok(l[1].to_string())
        }
    }
    return Err("Anonymous");
}


pub fn parse_for_errors(http_request: &Vec<String>) -> constants::HttpResponseStatus {
    if http_request.len() == 0 {
        return constants::HttpResponseStatus::NotFound;
    }
    if ! http_request[0].contains("GET / HTTP/1.1"){
        return constants::HttpResponseStatus::NotFound;
    }
    return constants::HttpResponseStatus::Ok;
}