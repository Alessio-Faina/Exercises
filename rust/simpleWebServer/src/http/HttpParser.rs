
pub fn parseForAgent(http_request: &Vec<String>) -> Result<String, &'static str> {
    for item in http_request {
        if item.contains("User-Agent") {
            let l : Vec<&str> = item.split(" ").collect();
            return Ok(l[1].to_string())
        }
    }
    return Err("Anonymous");
}


pub fn parseForErrors(http_request: &Vec<String>) -> i16 {
    if http_request.len() == 0 {
        return 404;
    }
    if ! http_request[0].contains("GET / HTTP/1.1"){
        return 404;
    }
    return 200;
}