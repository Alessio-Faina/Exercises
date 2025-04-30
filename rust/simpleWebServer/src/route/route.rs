use std::vec;
use std::sync::{LazyLock, Mutex};

use crate::http::constants::HttpResponseStatus;
use crate::http::http_parser;

pub enum RouteResolverError {
    CannotFindPath
}

pub enum RouteAddError {
    InvalidRoute,
    RouteAlreadyExists
}

pub struct Route {
   route: String,
   error_code : HttpResponseStatus,
   responder: fn(&Vec<String>) -> String
}

static ROUTES: LazyLock<Mutex<Vec<Route>>> = LazyLock::new(|| Mutex::new(vec![]));

fn internal_safe_add_a_route(route: Route) {
    ROUTES.lock().unwrap().push(route);
}

fn internal_safe_get_route_by_path(path: String) -> Option<fn(&Vec<String>) -> String> {
    for item in ROUTES.lock().unwrap().iter() {
        if item.route == path {
            return Some(item.responder)
        }
    }
    return None
}

fn internal_safe_get_route_by_error_code(err_code: HttpResponseStatus) -> Option<fn(&Vec<String>) -> String> {
    for item in ROUTES.lock().unwrap().iter() {
        if item.error_code == err_code {
            return Some(item.responder)
        }
    }
    return None
}

fn call_route_by_path(path: String, 
                          http_request: &Vec<String>) -> Result<String, RouteResolverError> {
    match internal_safe_get_route_by_path(path) {
        Some(func_dispatcher) => { return Ok((func_dispatcher)(http_request)) },
        None => { return Err(RouteResolverError::CannotFindPath);}
    }
}

fn call_route_by_error_code(err_code: HttpResponseStatus, 
                                http_request: &Vec<String>) -> Result<String, RouteResolverError> {
    
    match internal_safe_get_route_by_error_code(err_code) {
        Some(func_dispatcher) => { return Ok((func_dispatcher)(http_request)) },
        None => { return Err(RouteResolverError::CannotFindPath);}
    }
}

pub fn add_route_by_path(route_d: String, responder_f: fn(&Vec<String>) -> String) {
    let r = Route { route: route_d, error_code: HttpResponseStatus::Ok, responder: responder_f };
    internal_safe_add_a_route(r);
}

pub fn add_route_by_error_code(err_code: HttpResponseStatus, responder_f: fn(&Vec<String>) -> String) {
    let r = Route { route: "".to_string(), error_code: err_code, responder: responder_f };
    internal_safe_add_a_route(r);

}

pub fn call_route(http_request: &Vec<String>) -> String {
    let mut response: String= "".to_string();

    match http_parser::parse_for_errors(&http_request) {
        HttpResponseStatus::Ok => { 
            let route = http_parser::get_route(&http_request);
            let call_result = call_route_by_path(route, &http_request);
            match call_result {
                Ok(res) => { response += &res },
                Err(_) => { response += "INTERNAL ERROR"}
            }
        }
        HttpResponseStatus::NotFound => { 
            let call_result = call_route_by_error_code(HttpResponseStatus::NotFound, &http_request);
            match call_result {
                Ok(res) => {response += &res },
                Err(_) => { response += "INTERNAL ERROR" }
            }
        }
    }
    return response;
}
