use std::env;

fn main() {
    let var = "27".to_string();
    let v_int = var.parse::<i32>().unwrap();
    let v_int1 : i32 = var.parse().unwrap();
    let var_str = "27.1".to_string();
    let v_f = var_str.parse::<f32>().unwrap();
    let v_f1 : f32 = var_str.parse().unwrap();
    let args: Vec<String> = env::args().skip(1).collect();

    println!("{} {} {} {}", v_int, v_int1, v_f, v_f1);

    for arg in args {
        let v : i32 = arg.parse().unwrap();
        println!("{}", v);
    }
}
