# luhnr

A simple, but efficient, luhn number generator and validator for Rust. 

I wrote this library as I couldn't find a performant mod10 library available for Rust.

## Usage

[API Docs](https://docs.rs/luhnr/0.3.4/luhnr/)

_Note: the library is written for the performant path to be the functions that accept a slice of u8's:_

- _`validate(number: &[u8])`_
- _`generate_with_prefix(length: usize, prefix: &[u8])`_
- _`generate(length: usize)`_

_The `_str` methods are exponentially slower due to having to additonal allocations, and are only provided only as convenience:_

- _`validate_str(number: &str)`_
- _`generate_with_prefix_str(length: usize, prefix: &str)`_
- _`generate_str(length: usize)`_

### Quick Start

### Validate Luhn Number

Validate will return `true` if the vector of numbers passes Luhn's algorithm, or `false` if it fails.

```Rust
use luhnr;

fn main() {
  let number = vec![4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2];
  println!("The number valiates as: {}", luhnr::validate(&number));
}
```

### Generate

Generate will generate a luhn compliant number, thats meets the length passed in.

```Rust
use luhnr;

fn main() {
  match luhnr::generate(16) {
    Ok(v) => println!("The number is: {:?}", v),
    Err(e) => println!("recieved error: {:?}", e),
  }
}
```

Or pass a prefix, and use generate_with_prefix:

```Rust
use luhnr;

fn main() {
  let prefix = [4, 2, 4, 2, 4, 2];
  match luhnr::generate_with_prefix(16, &prefix) {
    Ok(v) => println!("The number is: {:?}", v),
    Err(e) => println!("recieved error: {:?}", e),
  }
}
```
