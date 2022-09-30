enum Fruits {
    Apple,
    Orange,
}

fn get_string(fruit: Fruits) -> String {
    match fruit {
        Fruits::Apple => "Apple".to_string(),
        Fruits::Orange => "Orange".to_string(),
    }
}

fn main() {
    let f = Fruits::Apple;
    let f_s = get_string(f);

    println!("fruit {}", f_s);
}
