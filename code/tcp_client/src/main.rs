fn tcp_client() {
    use std::net::TcpStream;

    let mut conn = TcpStream::connect("127.0.0.1:4444");

    match conn {
        Ok(conn) => {
            println!("connected");
        }
        Err(e) => {
            println!("connection failed");
        }
    }
}

fn main() {
    tcp_client();
}
