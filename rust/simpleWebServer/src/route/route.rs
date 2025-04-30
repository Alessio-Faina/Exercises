pub enum RouteResolverError {
    CannotFindPath
}

pub struct Route {
   route: String,
   responder: fn(&Vec<String>) -> String
}

pub fn add_route(route_d: String, responder_f: fn(&Vec<String>) -> String) -> Route {
    return Route { route: route_d, responder: responder_f };
}
/*
pub fn call_route(path: String, routes: Vec<Route>) -> Result<String, RouteResolverError> {
    match routes.into_iter() {

    }
}
     */
