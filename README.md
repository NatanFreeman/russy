# russy
`russy` is a utility crate for adding "ussy" to the end of things.

## Usage
```rust
use russy::mussy::fussy;

fn main(){
    let rust = "rust".to_string();
    assert_eq!("russy", fussy(&rust));
}
```

## Limitations

Only works with Roman alphanumeric characters.

## License

This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)