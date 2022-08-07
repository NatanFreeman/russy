//!# russy
//!`russy` is a utility crate for adding "ussy" to the end of things.
//!
//!## Usage
//!```rust
//!use russy::mussy::fussy;
//!
//!fn main(){
//!    let rust = "rust".to_string();
//!    assert_eq!("russy", fussy(&rust));
//!}
//!```
//!
//!## Limitations
//!
//!Only works with Roman alphanumeric characters.
//!
//!## License
//!
//!This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
use std::io::BufRead;
use std::{fs::File, io, path::Path};
pub mod mussy;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
#[allow(unused)]
pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::mussy::fussy;
    use crate::read_lines;

    #[test]
    fn tested_words() {
        let lines = read_lines("test_words.txt").unwrap();

        let expected_results = lines
            .map(|x| {
                let x = x.unwrap();
                let index = x.find("-").unwrap();
                (x[..index].to_string(), x[index + 1..].to_string())
            })
            .collect::<Vec<(String, String)>>();

        expected_results
            .iter()
            .for_each(|x| assert_eq!(x.1, fussy(&x.0)));
    }
}
