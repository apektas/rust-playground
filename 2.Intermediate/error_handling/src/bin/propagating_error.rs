use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    let content = read_file("example.txt");
    match content {
        Ok(content) => println!("File Read successfully"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound  => println!("File not found"),
                _ =>  println!("Error : {}", err),
            }
        }
    }

    let mut user: User = User{
        firstname: "abdurrahman".to_string(),
        lastname: "pektas".to_string()
    };

    println!("Short version: {:?}", get_initials(&user));

    println!("##############");

    // Struct Update Syntax
    user = User{
        firstname: "".to_string(),
        ..user
    };

    println!("Short version: {:?}", get_initials(&user));
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    // propagate the error using ? operator
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}


struct User {
    firstname: String,
    lastname: String
}

// ? operator can be also applied to Option type.
// If None variant occurred then return immediately.
fn get_initials(user: & User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?;
    println!("If None variant then this line not executed.");
    let last_initial = user.lastname.chars().next()?;
    println!("If None variant then this line not executed.");
    Some(format!("{first_initial}.{last_initial}."))
}