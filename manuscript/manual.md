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
