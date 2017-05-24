# EnumIndex
EnumIndex is a trait to extract variant index from enums in Rust.  
IndexEnum is a trait that allows you to get an enum variant by indexing. It only  works if all parameters of Enum implement Default


The main case is implementation of your own enum serialization.

```rust
pub trait EnumIndex {
    fn enum_index(&self) -> usize;
}

pub trait IndexEnum {
    fn index_enum(index: usize) -> Option<Self> where Self: Sized;
}

```

# Example

```rust
extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
use enum_index::{EnumIndex, IndexEnum};

// IndexEnum can only be derived for enums with parameters implementing Default
// In this example Default is checked for String, f32 and u64.
#[derive(EnumIndex, IndexEnum, Debug)]
enum OpCode {
    Disconnect,                       // 0
    SendData(String),                 // 1
    SomethingElse { a: f32, b: u64 }  // 2
}

fn main() {
    let first = OpCode::Disconnect;
    let second = OpCode::SendData("hello");
    let third = OpCode::SomethingElse {a: 1f32, b: 2u64};
    println!("{}", first.enum_index()); // prints 0
    println!("{}", second.enum_index()); // prints 1
    println!("{}", third.enum_index()); // prints 2

    let send_data_variant = OpCode::index_enum(2).unwrap();
    println!("{:?}", send_data_variant); // prints SomethingElse{a: 0f32, b: 0u64}
}
```

# Using in your project
Add the following in your Cargo.toml file
```toml
[dependencies]
enum_index = "0.2.0"
enum_index_derive = "0.2.0"
```

Then import needed trait and macro
```rust
// Strum contains all the trait definitions
extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
```
