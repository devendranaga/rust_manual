use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_millis(1000);
    let one_second_2 = time::Duration::from_secs(1);
    let one_millisecond = time::Duration::from_millis(100);
    let one_microsecond = time::Duration::from_micros(1000000);

    println!("test program 1");
    thread::sleep(one_second);
    println!("test program 2");

    thread::sleep(one_second_2);
    thread::sleep(one_millisecond);
    thread::sleep(one_microsecond);
}
