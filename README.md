# luhnr

- [luhnr](#luhnr)
  - [Usage](#usage)
    - [Quick Start](#quick-start)
    - [Validate Luhn Number](#validate-luhn-number)
    - [Generate](#generate)
  - [Benchmarks](#benchmarks)


A simple, but efficient, luhn number generator and validator for Rust. 

I wrote this library as I couldn't find a performant mod10 library available for Rust.

## Usage

[API Docs](https://docs.rs/luhnr/0.3.4/luhnr/)

Note: the library is written for the performant path to be the functions that accept a slice of u8's:

- `validate(number: &[u8])`
- `generate_with_prefix(length: usize, prefix: &[u8])`
- `generate(length: usize)`

The `_str` methods are exponentially slower due to having to additional allocations, and are only provided as convenience:

- `validate_str(number: &str)`
- `generate_with_prefix_str(length: usize, prefix: &str)`
- `generate_str(length: usize)`

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

## Benchmarks

Criterion benchmarks are provided in [./benches](benches/). 

On an Intel Core i9, MacBook Pro:

```
generate                time:   [133.52 ns 133.93 ns 134.41 ns]
generate_str            time:   [1.1067 µs 1.1201 µs 1.1368 µs]
```
