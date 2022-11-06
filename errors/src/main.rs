use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    let username = read_username_from_file();
    println!("{:?}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
