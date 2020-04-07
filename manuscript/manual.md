# Rust

## Introduction

**Helloworld**

```rust
fn main() {
    println!("Hello world\n");
}
```

## cargo

**create new project:**

```bash
cargo new <project-name>
```

```bash
cd <project-name>
cargo build
```

## types

1. signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
2. unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

```rust

let def = "a string";

def = "new string";

```

changing the variable to other name, would result in a compiler error. Rust variables are by default mutable. Changing the value results in a compiler error.

## conditional

```rust
if condition {
.. statements ..
} else {
.. statements ..
}
```


### character type

1. is_lowercase:

```rust

let mut char_types = 'c';

let res = char_types.is_lowercase();
if res {
    println!("is lowercase");
} else {
    println!("is uppercase");
}

```

2. is_alphabet:

```rust
let mut char_types = 'c';

let res = char_types.is_alphabet();
if res {
    println!("is alphabet");
} else {
    println!("is not an alphabet");
}

```

3. is_alphanumeric:

```rust
let alnum = '7';

let res = alnum.is_alphanumeric();
if res {
    println!("is alphanumeric");
} else {
    println!("is not alphanumeric");
}

```

## std library

### std::process

**std::process::id**:

```rust
use std::process

println!("the pid : {}", process::id());

```

**struct std::process::Command**:

This structure is used to represent and maange child proc. A child proc is created via this structure. The calling convention is similar to the builder pattern.

members exposed by `Command`:

| member function | description |
|-----------------|-------------|
| `new` | create new process |
| `arg` | argument |
| `spawn` | spanw the process |
| `expect` | expect the child process failure or success |


```rust
use std::process;

let child = process::Command::new("/bin/ls")
                    .spawn()
                    .expect("failed to execute ls");

let output = child
            .wait_with_output()
            .expect("failed to wait on child process");

let res = output.status.success();
println!("{}", res);

```

the arguments are passed via the `arg`.

```rust

use std::process;

let child = process::Command::new("/bin/ls")
                    .arg("-l")
                    .spawn()
                    .expect("failed to execute ls");

let output = child
            .wait()
            .expect("failed to wait on child process");

let res = output.status.success();
println!("{}", res);

```

always `wait` must be called in a long running process so that process that's created is cleanedup.
