fn main() {
    let a:u32 = 4;
    let b:u32 = 2;

    let result = a & b;
    println!("result & {}", result);

    let result = a | b;
    println!("result | {}", result);

    let result = a ^ b;
    println!("result ^ {}", result);

    let result = a << 2;
    println!("result << {}", result);

    let result = !a;
    println!("result ! {}", result);

    let result = a >> 2;
    println!("result >> {}", result);

    let result = a == 4;
    println!("result == {}", result);
}
