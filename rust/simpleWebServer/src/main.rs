use std::{
    /*fmt::Error,*/ io::{prelude::*, BufReader/* , Empty*/}, net::{TcpListener, TcpStream}
};

mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


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
        200 => { response = "HTTP/1.1 200 OK\r\n\r\n Hello ".to_string()}
        404 => { 
                response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
                stream.write_all(response.as_bytes()).unwrap(); 
                return;
            }
        i16::MIN..=199_i16 | 201_i16..=403_i16 | 405_i16..=i16::MAX => todo!()
    } 

    match http::http_parser::parse_for_agent(&http_request) {
        Ok(v) => {response += &v},
        Err(e) => {response += &e}
    }

    response += " user !\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
