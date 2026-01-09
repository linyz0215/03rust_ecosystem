use error_study::MyError;
use std::{any, fs};
use anyhow::Context;
use std::num::ParseIntError;
fn main() -> Result<(), MyError> {
    println!("size of anyhow::Error is {}", size_of::<anyhow::Error>());
    println!("size of std::io::Error is {}", size_of::<std::io::Error>());
    println!(
        "size of std::num::ParseIntError is {}",
        size_of::<ParseIntError>()
    );
    println!(
        "size of serde_json::Error is {}",
        size_of::<serde_json::Error>()
    );
    println!("size of string is {}", size_of::<String>());
    println!("size of MyError is {}", size_of::<MyError>());
    let filename = "NonExistentFile.txt";
    match fs::File::open(filename).map_err(MyError::from) {
        Ok(content) => println!("File content: {:?}", content),
        Err(e) => println!("{}",  e),
    }
    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}