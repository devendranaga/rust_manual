fn main() {
    let apple = "apple".to_string();
    let banana = "banana".to_string();
    let mut var = String::new();

    println!("{}", apple == banana);
    println!("{}", apple.eq(&banana));

    //var = apple + " " + &banana;

    let mut var2 = [apple, banana].join(" ");

    println!("{}", var2);
}
