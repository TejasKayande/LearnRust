
// imorting external modules
use std::fs; // for file system operations
use std::collections::*; // import all items from the collections module
use std::io::{Write, Read}; // import specific items from the io module

// modules
mod math {

    // we can have functions, structs, enums, and even other modules inside a
    // module.

    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }
    
    pub fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    
    pub fn divide(x: i32, y: i32) -> i32 {
        x / y
    }

    pub mod advanced {
        pub fn double(x: i32) -> i32 {
            // refering the parent modult using 'super'
            super::add(x, x)
        }
    }
}

// can alias a module or function to avoid name conflicts or for convenience.
use math::subtract as sub; 

fn main() -> () {

    // absolute path. here 'create' refers to the root of the current crate.
    let sum = crate::math::add(5, 3);    
    println!("The sum is: {}", sum);

    // but because 'math' is a sibling of 'main', we can also use a relative path.
    let difference = math::subtract(5, 3);
    println!("The difference is: {}", difference);

    use math::advanced::double; // we can put this anywhere in the code. Even after where its used!

    // we bring 'double' in scope, so we can use it without the full path.
    let double = double(difference);
    println!("The double of {} is: {}", difference, double);

}
