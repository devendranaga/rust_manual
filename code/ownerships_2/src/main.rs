fn main() {
    let var = Box::new(4u32);

    println!("{}", var);
    let mut var1 = var;

    *var1 = 14;
    println!("{}", var1);
}
