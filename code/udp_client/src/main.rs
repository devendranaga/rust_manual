use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:1244").unwrap();
    let send_buf : String = String::from("hello");

    socket.send_to(send_buf.as_bytes(), "127.0.0.1:1245").unwrap();
}
