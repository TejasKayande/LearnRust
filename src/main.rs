
fn main() -> () {
    
    let mut _s = String::new(); // new empty string

    let mut s = String::from("Hello, world!");
    println!("{}", s);

    s.push_str(" Welcome to Rust programming.");
    println!("{}", s);

    let litral = "This is a string literal.";
    println!("{}", litral);

    let litral_to_string = litral.to_string();
    println!("{}", litral_to_string);

    let hello = String::from("नमस्ते"); // supports Unicode characters
    println!("{}", hello);

    let s2 = s + &hello; // s was moved here 
    println!("{}", s2);
    // println!("{}", s); // because s was moved, this line would cause a compile error

    // instead we could use format! macro to concatenate without moving ownership
    let s3 = format!("{} {}", s2, hello);
    println!("{}", s3);
    println!("{}", s2); // We can still use s2 after the use of format! 

    // you cannot index into a String with an integer because of UTF-8 encoding,
    // which can have variable byte lengths for characters.
    // s3[0] 

    // iterating over characters in a string
    for c in s3.chars() {
        print!("{}", c);
    }
    println!();

    // review previous commits for slicing strings and string literals 

}
