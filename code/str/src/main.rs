fn main() {

    let string = "rustlang on fc36";
    let emptystr = "";
    let len = string.len(); // get length of the string

    println!("length {}", len);

    println!("empty str: {} {}", string.is_empty(), emptystr.is_empty());
}
