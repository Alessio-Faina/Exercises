use crate::http::constants::HttpResponseStatus;

pub enum RouteResolverError {
    CannotFindPath
}

pub struct Route {
   route: String,
   error_code : HttpResponseStatus,
   responder: fn(&Vec<String>) -> String
}

pub fn add_route_by_path(route_d: String, responder_f: fn(&Vec<String>) -> String) -> Route {
    return Route { route: route_d, error_code: HttpResponseStatus::Ok, responder: responder_f };
}

pub fn add_route_by_error_code(err_code: HttpResponseStatus, responder_f: fn(&Vec<String>) -> String) -> Route {
    return Route { route: "".to_string(), error_code: err_code, responder: responder_f };
}

pub fn call_route_by_path(path: String, 
                          routes: &Vec<Route>,
                          http_request: &Vec<String>) -> Result<String, RouteResolverError> {
    for item in routes {
        if item.route == path {
            return Ok((item.responder)(http_request))
        }
    }
    Err(RouteResolverError::CannotFindPath)
}

pub fn call_route_by_error_code(err_code: HttpResponseStatus, 
                                routes: &Vec<Route>,
                                http_request: &Vec<String>) -> Result<String, RouteResolverError> {
    for item in routes {
        if item.error_code == err_code {
            return Ok((item.responder)(http_request))
        }
    }
    Err(RouteResolverError::CannotFindPath)
}