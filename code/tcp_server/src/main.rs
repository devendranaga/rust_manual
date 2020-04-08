fn tcp_server() {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:4444").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client");
            }
            Err(e) => {
                println!("connection fail");
            }
        }
    }
}

fn main() {
    tcp_server();
}
