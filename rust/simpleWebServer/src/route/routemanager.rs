use std::collections::HashMap;
use crate::http::constants;
use crate::http::constants::HttpResponseStatus;
use crate::http::http_parser;


#[allow(dead_code)]
struct Route {
    route: String,
    error_code : HttpResponseStatus,
    responder: fn(&Vec<String>) -> String
 }
 
 impl Route {
     pub fn new(route: String, error_code : HttpResponseStatus, responder: fn(&Vec<String>) -> String) -> Route {
         return Route {route, error_code, responder};
     }
 }

pub enum RouteResolverError {
    CannotFindPath
}

/*
pub enum RouteAddError {
    InvalidRoute,
    RouteAlreadyExists
}
*/

pub struct RouteManager {
    path_routes : HashMap<String, Route>,
    error_routes : HashMap<constants::HttpResponseStatus, Route>
}

impl RouteManager {
    pub fn new() -> RouteManager {
        return RouteManager{path_routes : HashMap::new(), error_routes : HashMap::new() };
    }

    pub fn add_route_by_path(&mut self, route_d: String, responder_f: fn(&Vec<String>) -> String) {
        if self.path_routes.contains_key(&route_d) {
            // We should return an error
        } else {
            let s: String = route_d.clone();
            let r = Route::new(route_d, constants::HttpResponseStatus::Ok, responder_f);
            self.path_routes.insert(s, r);
        };
    }
    
    pub fn add_route_by_error_code(&mut self, err_code: constants::HttpResponseStatus, responder_f: fn(&Vec<String>) -> String) {
        if self.error_routes.contains_key(&err_code) {
            // We should return an error
        } else {
            let s: constants::HttpResponseStatus = err_code.clone();
            let r = Route::new("".to_string(), err_code, responder_f);
            self.error_routes.insert(s, r);
        };
    }

    pub fn call_route_by_path(&self, path: String, http_request: &Vec<String>) -> Result<String, RouteResolverError> {
        let res: Option<&Route> = self.path_routes.get(&path);
        match res {
            Some(item) => return Ok((item.responder)(http_request)),
            None => return self.call_route_by_error_code(HttpResponseStatus::NotFound, http_request)   
        }
    }

    pub fn call_route_by_error_code(&self, err_code: HttpResponseStatus, http_request: &Vec<String>) -> Result<String, RouteResolverError> {
        let res: Option<&Route> = self.error_routes.get(&err_code);
        match res {
            Some(item) => return Ok((item.responder)(http_request)),
            None => return Err(RouteResolverError::CannotFindPath)
        }
    }

    pub fn call_route(&self, http_request: &Vec<String>) -> String {
        let mut response: String= "".to_string();
    
        match http_parser::parse_for_errors(&http_request) {
            HttpResponseStatus::Ok => { 
                let route = http_parser::get_route(&http_request);
                let call_result = self.call_route_by_path(route, &http_request);
                match call_result {
                    Ok(res) => { response += &res },
                    Err(_) => { response += constants::HTTP_INTERNAL_ERROR }
                }
            }
            HttpResponseStatus::NotFound => { 
                let call_result = self.call_route_by_error_code(HttpResponseStatus::NotFound, &http_request);
                match call_result {
                    Ok(res) => {response += &res },
                    Err(_) => { response += constants::HTTP_INTERNAL_ERROR }
                }
            }
        }
        return response;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::route;

    #[test]
    fn no_path_defined_for_errors_returns_internal_error() {
        let http_request : Vec<String> = vec!["GET / HTTP/1.1".to_string()];
        let rm = RouteManager::new();
        assert_eq!(rm.call_route(&http_request), constants::HTTP_INTERNAL_ERROR);
    }

    #[test]
    fn no_path_defined_but_404_defined_returns_404_err() {
        let http_request : Vec<String> = vec!["GET / HTTP/1.1".to_string()];
        let mut rm = RouteManager::new();
        rm.add_route_by_error_code(HttpResponseStatus::NotFound, route::not_found::get_response);
        assert_eq!(rm.call_route(&http_request).contains(constants::HTTP_NOT_FOUND_HEADER), true);
    }

    #[test]
    fn path_defined_and_requested_returns_200_ok() {
        let http_request : Vec<String> = vec!["GET / HTTP/1.1".to_string()];
        let mut rm = RouteManager::new();
        rm.add_route_by_path("/".to_string(), route::index::get_response);
        rm.add_route_by_error_code(HttpResponseStatus::NotFound, route::not_found::get_response);
        assert_eq!(rm.call_route(&http_request).contains(constants::HTTP_SUCCESS_HEADER), true);
    }

    #[test]
    fn path_defined_with_parameters_and_requested_returns_200_ok() {
        let http_request : Vec<String> = vec!["GET /?name=mike&surname=shumi HTTP/1.1".to_string()];
        let mut rm = RouteManager::new();
        rm.add_route_by_path("/".to_string(), route::index::get_response);
        rm.add_route_by_error_code(HttpResponseStatus::NotFound, route::not_found::get_response);
        assert_eq!(rm.call_route(&http_request).contains(constants::HTTP_SUCCESS_HEADER), true);
    }
}
