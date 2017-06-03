# init\_with

[Documentation][]

[Documentation]: https://docs.rs/init_with

Have you wanted to be able to initialize a fixed array in Rust by calling a function to create each
element? Now you can!

```rust
use init_with::InitWith;

let my_array = {
    let mut seed = Vec::new();
    let mut next_val = 0;

    <[Vec<u32>; 3]>::init_with(|| {
        seed.push(next_val);
        next_val += 1;
        seed.clone()
    })
};

assert_eq!(my_array, [vec![0], vec![0, 1], vec![0, 1, 2]]);
```

This crate lets you initialize the array elements in a functional manner while hiding the unsafe
code that's needed to do so.

To import this crate, put the following into your Cargo.toml:

```toml
[dependencies]
init_with = "1.0.2"
```

...and the following in your crate root:

```rust
extern crate init_with;
```
