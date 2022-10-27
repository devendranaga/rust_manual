use std::str;

fn main() {
    let buf = [104, 101, 108, 108, 111];
    let s = str::from_utf8(&buf).unwrap().to_string();
    let out_buf = s.as_bytes();

    println!("{}", s);

    let mut i = 0;

    while i < out_buf.len() {
        println!("{}", out_buf[i]);
        i += 1;
    }
}
