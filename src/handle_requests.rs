use crate::srvrio::{load_html, read_binary_data};
use std::fmt::{Result, format};
use std::io::{BufRead, BufReader, ErrorKind, Write};
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
        let mut html_data = String::new();
        match load_html(&file_path) {
            Ok(string) => html_data = string,
            Err(E) => {
                if E.kind() == ErrorKind::NotFound {
                    let not_found_page = format!("{root_dir}404.html");
                    html_data = load_html(&not_found_page).unwrap();
                }
            }
        }
        let content_length = html_data.len();
        let response =
            format!("{OK_STATUS}\r\nContent-Length: {content_length}\r\n\r\n{html_data}");
        print!("{}", &response);
        stream.write_all(response.as_bytes());
    } else if request.ends_with(".html") || request.ends_with(".css") || request.ends_with(".js") {
        let mut URI = request;
        _ = URI.remove(0);
        let URI = format!("{root_dir}{URI}");
        let mut html_data = String::new();
        match load_html(&URI) {
            Ok(string) => html_data = string,
            Err(E) => {
                let not_found_page = format!("{root_dir}404.html");
                html_data = load_html(&not_found_page).unwrap();
            }
        }
        let content_length = html_data.len();
        let response =
            format!("{OK_STATUS}\r\nContent-Length: {content_length}\r\n\r\n{html_data}");
        print!("{}", &response);
        stream.write_all(response.as_bytes());
    } else {
        let mut URI = request;
        _ = URI.remove(0);
        let URI = format!("{root_dir}{URI}");
        let mut image_vec: Vec<u8> = Vec::new();
        match read_binary_data(&URI) {
            Ok(data) => image_vec = data,
            Err(_) => return,
        }
        let content_length = image_vec.len();
        let file_type = URI.split(".").last().unwrap();
        let content_type = format!("image/{file_type}");
        let response = format!(
            "{OK_STATUS}\r\nContent-Type {content_type}\r\nContent-Length {content_length}\r\n\r\n"
        )
        .into_bytes();
        stream.write_all(&response);
        stream.write_all(&image_vec);
    }
}
