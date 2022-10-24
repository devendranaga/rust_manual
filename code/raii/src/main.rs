fn create_str() {
    let mut _str = String::new();

    _str = "hello".to_string();

    println!("str {}", _str);
}

fn main() {
    create_str();
}
