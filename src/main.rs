
fn main() {

    // Variables 
    let mut x: u32 =  5; //  mutable 32 bit integer variable
    let y: u64 = 10; // immutable 64 bit integer variable

    // NOTE(Tejas): difference between immutable and constant variables is that
    // immutable variables are assigned a value at runtime (maybe through a
    // function call) and can be changed, while constant variables are fixed at
    // compile time and cannot be changed.

    // const values must be know at compile time
    const PI: f64 = 3.14159; // constant variable

    // Data types
    /*
    Length                   Signed  Unsigned
    ------------------------------------------
    8-bit                    i8      u8
    16-bit                   i16     u16
    32-bit                   i32     u32
    64-bit                   i64     u64
    128-bit                  i128    u128
    Architecture-dependent   isize   usize
    */


    // NOTE(Tejas): Integer overflow will cause a panic in debug mode and if
    // compiled with --release flag, it will wrap around using two's complement
    // wrapping.

    println!("Integer Overflow and Wrapping: ");

    // NOTE(Tejas): compile with --release flag.
    let mut x: u8 = 255; // maximum value for u8
    println!("Before overflow: {}", x);

    // NOTE(Tejas): will cause overflow and panic in debug mode, but will wrap
    // around to 0 in release mode. Uncomment to see the effect in debug mode.
    // x = x + 1;

    println!("After overflow: {}", x); // will wrap around to 0


    // NOTE(Tejas): We can additionally use the wrapping_add method to explicitly handle overflow.
    // NOTE(Tejas): doesnt need --release flag, will always wrap around.
    let mut x: u8 = 255;
    println!("Before wrapping_add: {}", x);
    x = x.wrapping_add(1); // will handle overflow by wrapping around
    println!("After wrapping_add: {}", x); // will wrap around to 0
    println!("--------------------------------------------------");


    // compound data types
    // tuples
    println!("Tuples: ");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring a tuple
    println!("Accession using Destruction: ");
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    println!("Accession using Index: ");
    println!("The value of the first element is: {}", tup.0);
    println!("The value of the second element is: {}", tup.1);
    println!("The value of the third element is: {}", tup.2);
    println!("--------------------------------------------------");


    println!("Arrays: ");
    let arr: [i32; 5] = [123, 22, 35, 41, 58]; // array of 5 integers
    for i in 0..arr.len() {
        println!("Index: {} Value: {}", i, arr[i]);
    }

    println!("Filling an array with the same value: ");
    let arr = [4; 5];
    for i in 0..arr.len() {
        println!("Index: {} Value: {}", i, arr[i]);
    }
    println!("--------------------------------------------------");

}

