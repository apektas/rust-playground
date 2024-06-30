use std::{error, fs};
use std::num::ParseIntError;

fn main() {
    // let num = parse_file("example.txt");
    let num = parse_file2("example.txt");
    match num {
        Ok(number) => println!("Number is: {}", number),
        Err(e) => {
            match e {
                ParseFileError::FileError => {
                    print!("File error");
                },
                ParseFileError::ParseError( e) => {
                    print!("Parsing error: {}", e);
                }
            }
        }
    }
}


// different error types in one method is not allowed
/*
fn parse_file(file_name: &str) -> Result<i32, io::Error> {
    let content = fs::read_to_string(file_name)?;
    let parsed_value = content.parse()?;
    Ok(parsed_value)
}
 */

// Trait object resolves the issue
fn parse_file(file_name: &str) -> Result<i32, Box<dyn error::Error>> {
    let content = fs::read_to_string(file_name)?;
    let parsed_value = content.parse()?;
    Ok(parsed_value)
}

enum ParseFileError {
    FileError,
    ParseError(ParseIntError)
}

fn parse_file2(file_name: &str) -> Result<i32, ParseFileError> {
    let content = fs::read_to_string(file_name)
        .map_err(|_| ParseFileError::FileError)?;
    let parsed_value = content.parse()
        .map_err(|e| ParseFileError::ParseError(e))?;
    Ok(parsed_value)
}