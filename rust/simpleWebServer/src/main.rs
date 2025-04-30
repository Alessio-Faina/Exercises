use std::{
    /*fmt::Error,*/ io::{prelude::*, BufReader/* , Empty*/}, net::{TcpListener, TcpStream}
};

mod http;
use http::{constants::HttpResponseStatus, http_parser};
mod route;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut routes : Vec<route::route::Route> = vec![];
    routes.push(route::route::add_route("/".to_string(),
                route::index::get_response));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
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
        HttpResponseStatus::Ok => { response = "HTTP/1.1 200 OK\r\n\r\nHello ".to_string()}
        HttpResponseStatus::NotFound => { 
                response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
                stream.write_all(response.as_bytes()).unwrap(); 
                return;
            }
    } 

    match http::http_parser::parse_for_agent(&http_request) {
        Ok(v) => {response += &v},
        Err(e) => {response += &e}
    }

    response += " user !\r\n";
    response += "You requested route ";
    response += &http_parser::get_route(&http_request);
    response += "\r\n";

    let a = http_parser::get_request_parameters(&http_request);
    for item in a {
        response += &item;
        response += "\r\n";
    }

    response += "\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
