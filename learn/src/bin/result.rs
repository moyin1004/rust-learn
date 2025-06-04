use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
    let mut buf = String::new();
    match greeting_file.read_to_string(&mut buf) {
        Ok(_) => println!("{}", buf),
        Err(e) => panic!("Problem reading the file: {e:?}"),
    }
    println!("{greeting_file:?}");
}
