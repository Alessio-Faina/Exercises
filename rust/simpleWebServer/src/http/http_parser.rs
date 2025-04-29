use regex::Regex;
use super::constants;


pub fn parse_for_agent(http_request: &Vec<String>) -> Result<String, &'static str> {
    for item in http_request {
        if item.contains("User-Agent") {
            let l : Vec<&str> = item.split(" ").collect();
            if l.len() < 2 {
                return Err("Anonymous");
            }
            return Ok(l[1].to_string())
        }
    }
    return Err("Anonymous");
}


pub fn parse_for_errors(http_request: &Vec<String>) -> constants::HttpResponseStatus {
    if http_request.len() == 0 {
        return constants::HttpResponseStatus::NotFound;
    }
    let re = Regex::new(r"(GET)\s(.+)\s(HTTP\/\d+\.\d+)").unwrap();
    if re.captures_len() == 0 {
        return constants::HttpResponseStatus::NotFound;
    }

    //if ! http_request[0].contains("GET / HTTP/1.1"){
    //    return constants::HttpResponseStatus::NotFound;
    //}
    return constants::HttpResponseStatus::Ok;
}

pub fn get_route(http_request: &Vec<String>) -> String {
    // Taken for granted, the GET / part has already been successful
    let l : Vec<&str> = http_request[0].split(" ").collect();
    let route = l[1];
    if route.contains("?") {
        let sanitised_route : Vec<&str>  = route.split("?").collect();
        return sanitised_route[0].to_string();
    }
    return route.to_string();
}

pub fn get_request_parameters(http_request: &Vec<String>) -> Vec<String> {
    // Taken for granted, the GET / part has already been successful
    let mut param_list : Vec<String> = vec![];
    let l : Vec<&str> = http_request[0].split(" ").collect();
    let route = l[1];
    if route.contains("?") {
        let sanitised_route : Vec<&str>  = route.split("?").collect();
        let params : Vec<&str> = sanitised_route[1].split("&").collect();
        for item in params {
            param_list.push(item.to_string());
        }
    }
    return param_list;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_user_agent_returns_anonymous() {
        let v : Vec<String> = vec!["User-Agent".to_string()];
        assert_eq!(parse_for_agent(&v), Err("Anonymous"));
    }

    #[test]
    fn empty_request_returns_404() {
        let v : Vec<String> = vec![];
        assert_eq!(parse_for_errors(&v), constants::HttpResponseStatus::NotFound);
    }

    #[test]
    fn basic_request_returns_200() {
        let v : Vec<String> = vec!["GET / HTTP/1.1".to_string()];
        assert_eq!(parse_for_errors(&v), constants::HttpResponseStatus::Ok);
    }

    #[test]
    fn request_with_params_returns_200() {
        let v : Vec<String> = vec!["GET /?name=1 HTTP/1.1".to_string()];
        assert_eq!(parse_for_errors(&v), constants::HttpResponseStatus::Ok);
    }

    #[test]
    fn request_with_multiple_params_returns_200() {
        let v : Vec<String> = vec!["GET /?name=1&name=2 HTTP/1.1".to_string()];
        assert_eq!(parse_for_errors(&v), constants::HttpResponseStatus::Ok);
    }

    #[test]
    fn request_with_no_params_returns_empty_vector() {
        let result: Vec<String> = vec![];
        let v : Vec<String> = vec!["GET / HTTP/1.1".to_string()];
        assert_eq!(get_request_parameters(&v), result);
    }

    #[test]
    fn request_with_multiple_params_returns_all_params() {
        let result = vec!["name=mike", "surname=shumi"];
        let v : Vec<String> = vec!["GET /?name=mike&surname=shumi HTTP/1.1".to_string()];
        assert_eq!(get_request_parameters(&v), result);
    }
}