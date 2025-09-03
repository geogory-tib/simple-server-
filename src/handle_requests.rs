use crate::srvrio::load_html;
use std::fmt::Result;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
const OK_STATUS: &str = "HTTP/1.1 200 OK";
pub fn handle_connetion(mut stream: TcpStream, root_dir: &str) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|str| !str.is_empty())
        .collect();
    print!("Request: {http_request:#?}");
    let http_method = http_request[0].clone();
    if http_method.contains("GET") {
        handle_get_request(http_method, &mut stream, root_dir);
    }
}
fn handle_get_request(http_method: String, stream: &mut TcpStream, root_dir: &str) {
    let request_vec: Vec<_> = http_method
        .split_whitespace()
        .filter(|string| *string != "GET" && !string.contains("HTTP"))
        .collect();
    let request = request_vec.concat();
    if request_vec[0] == "/" {
        let file_path = format!("{root_dir}index.html");
        let html_data = load_html(&file_path);
        let content_length = html_data.len();
        let response =
            format!("{OK_STATUS}\r\nContent-Length: {content_length}\r\n\r\n{html_data}");
        print!("{}", &response);
        stream.write_all(response.as_bytes());
    } else if request.ends_with(".html") {
        let mut URI = request;
        _ = URI.remove(0);
        let URI = format!("{root_dir}{URI}");
        let html_data = load_html(&URI);
        let content_length = html_data.len();
        let response =
            format!("{OK_STATUS}\r\nContent-Length: {content_length}\r\n\r\n{html_data}");
        print!("{}", &response);
        stream.write_all(response.as_bytes());
    }
}
