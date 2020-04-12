use std;

fn string_details(s : &String)
{
    println!("capacity {}", s.capacity());
    println!("empty {}", s.is_empty());
    if s.is_empty() == false {
        println!("\tstring {}", s);
    }
    println!("length {}", s.len());
}

fn main() {
    // create new empty string
    let mut s = String::new();
    string_details(&s);

    s.reserve(40);
    string_details(&s);

    // create a string on stack
    let s1 = "Hello";
    s.push_str(s1);
    string_details(&s);

    s.push_str(" rust programming");
    string_details(&s);

    let bytes = s.into_bytes();

    print!(" bytes ");
    for i in &bytes {
        print!("{} ", i);
    }
    println!("\n");
}
