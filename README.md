# Records
Records is a Rust library which adds an attribute designed for simple data classes ("records").

## What is a record?
The `record` attribute takes a standard named struct and
1. Makes all it's fields `pub`
2. Gives it a constructor
3. Implements convesrsion to/from tuples

## Example
```rust
#[records::record]
pub struct Person {
  name: String,
}

pub fn main() {
  let person = Person::new(String::from("World"));
  println!("Hello, {}!", person.name);
}
```

