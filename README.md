# EnumIndex
EnumIndex is a trait to extract variant index from enums in Rust

The main case is implementation of your own enum serialization.

```rust
pub trait EnumIndex {
    fn enum_index(&self) -> usize;
}
```

# Example

```rust
extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
use enum_index::EnumIndex;

#[derive(EnumIndex)]
enum Object {
    None,                    // 0
    Number(u64),             // 1
    Point {x: f32, y: f32}   // 2
}

fn main() {
    let first = Object::None;
    let second = Object::Number(0u64);
    let third = Object::Point {x: 0f32, y: 0f32};
    println!("{}", first.enum_index()); // prints 0
    println!("{}", second.enum_index()); // prints 1
    println!("{}", third.enum_index()); // prints 2
}
```

# Using in your project
Add the following in your Cargo.toml file
```toml
[dependencies]
enum_index = "*"
enum_index_derive = "*"
```

Then import needed trait and macro
```rust
// Strum contains all the trait definitions
extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
```
