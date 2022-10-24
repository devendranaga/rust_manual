
fn main() {
    let a = 4;
    let b = a; /* only the value is copied, not heap allocation */

    /* both a and b can be accessed here */
    println!("a: {} b: {}", a, b);

    let a1 = Box::new(4u32);

    let b1 = a1; /* now a1 pointer is moved to b1 and b1 owns the pointer */

    /* accessing a1 here results in compilation error */
    /* println!("a1: {} b1: {}", a1, b1); */
    println!("b1: {}", b1);
}
