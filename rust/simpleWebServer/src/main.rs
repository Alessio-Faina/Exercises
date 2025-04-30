use std::{
    /*fmt::Error,*/ io::{prelude::*, BufReader/* , Empty*/}, net::{TcpListener, TcpStream}
};

mod http;
use http::{constants::HttpResponseStatus, http_parser};
use route::route::{call_route_by_error_code, call_route_by_path};
mod route;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut routes : Vec<route::route::Route> = vec![];
    routes.push(route::route::add_route_by_path("/".to_string(),
                route::index::get_response));
    routes.push(route::route::add_route_by_error_code(HttpResponseStatus::NotFound,
                route::not_found::get_response));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &routes);
    }
}

fn handle_connection(mut stream: TcpStream, routes : &Vec<route::route::Route>) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    #[allow(unused_assignments)]
    let mut response: String= "".to_string();

    match http::http_parser::parse_for_errors(&http_request) {
        HttpResponseStatus::Ok => { 
            let route = http_parser::get_route(&http_request);
            let call_result: Result<String, route::route::RouteResolverError> = call_route_by_path(route, routes, &http_request);
            match call_result {
                Ok(res) => { response += &res },
                Err(_) => { response += "INTERNAL ERROR"}
            }
        }
        HttpResponseStatus::NotFound => { 
            let call_result: Result<String, route::route::RouteResolverError> = 
                    call_route_by_error_code(HttpResponseStatus::NotFound, routes, &http_request);
            match call_result {
                Ok(res) => {response += &res },
                Err(_) => { response += "INTERNAL ERROR" }
            }
        }
    } 

    response += "\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
