use std::{
    /*fmt::Error,*/ io::{prelude::*, BufReader/* , Empty*/}, net::{TcpListener, TcpStream}
};

mod http;
use http::constants::HttpResponseStatus;
mod route;
use route::routemanager::RouteManager;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut rm = RouteManager::new();
    rm.add_route_by_path("/".to_string(), route::index::get_response);
    rm.add_route_by_error_code(HttpResponseStatus::NotFound, route::not_found::get_response);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut rm);
    }
}

fn handle_connection(mut stream: TcpStream, routemanager: &mut RouteManager) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    #[allow(unused_assignments)]
    let mut response: String= routemanager.call_route(&http_request);

    response += "\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
