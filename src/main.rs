use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
mod handle_requests;
mod srvrio;
fn main() {
    let config_vec = srvrio::load_config();
    let root_dir = Arc::new(config_vec[config_vec.len() - 1].clone());
    let ip_and_port = format!("{0}:{1}", config_vec[0], config_vec[1]);
    drop(config_vec);
    let listener = TcpListener::bind(ip_and_port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let root_dir_clone = root_dir.clone();
        thread::spawn(move || {
            handle_requests::handle_connetion(stream, &root_dir_clone.as_str());
        });
    }
}
