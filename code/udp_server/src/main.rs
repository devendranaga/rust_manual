use std::str;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:1234").unwrap();
    let mut buf = [0; 100];

    let (amt, _src) = socket.recv_from(&mut buf).unwrap();
    let rx_data = &mut buf[..amt];
    let rx_msg = str::from_utf8(rx_data).unwrap().to_string();

    println!("{}", rx_msg);
}

