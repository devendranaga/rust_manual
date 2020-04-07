fn main() {
    use std::process;

    println!("my pid : {}", process::id());

    let child = process::Command::new("/bin/ls")
                            .spawn()
                            .expect("failed to exec child process");

    let output = child
                    .wait_with_output()
                    .expect("failed to wait on child process");
    let res = output.status.success();

    println!("{}", res);

    let child_with_arg = process::Command::new("/bin/ls")
                            .arg("-la")
                            .spawn()
                            .expect("Failed to exec child process");

    let output_with_arg = child_with_arg
                            .wait_with_output()
                            .expect("failed to wait on child process");
    let res_with_arg = output.status.success();

    println!("{}", res);
}
