# luhnr

A simple, but efficient, luhn number generator and validator for Rust.

## Usage

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
