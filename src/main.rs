
use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // handling errors with Result.

    let _file = File::open("hello.txt");

    let _file = match _file {
        Ok(file) => {
            println!("File opened successfully");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File created successfully");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        }
    };

    // we can instead use unwrap method, which returns the value inside Ok if
    // the result is Ok, but panics if the result is Err. We can also use expect
    // method, which is similar to unwrap but allows us to specify the panic
    // message.
    let _file = File::open("test.txt").unwrap();

    // additionally we can use .expect() method to provide a custom error
    // message if the file cannot be opened.
    let _file = File::open("test.txt").expect("Failed to open test.txt");

}