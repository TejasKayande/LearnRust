
fn main() {

    println!("Functions, Expressions, and Statements");

    // NOTE(Tejas): we can evaluate an expression in a block and assign the result to a variable.
    // NOTE(Tejas): underscore is used to indeicate that we know variable is unused.
    let _y = {
        let x = 3;

        // NOTE(Tejas): This is without a semicolon, so this will be returned,
        // if we add a semicolon, it becomes a statement.
        x + 1
    };

    println!("{}", another_function(32));
    println!("------------------------------");


    // Coditionals
    println!("Conditionals");
    let mut number = String::new();

    println!("Enter a number: ");
    std::io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let number: usize = number.trim().parse().expect("Please type a number!");

    // NOTE(Tejas): Rust does not allow one line if statements like in C
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    print!("Conditionals for expressions: ");
    let val = if number > 10 { "greater than 10" } else { "less than or equal to 10" };
    println!("Your number {} is {}", number, val);

    // NOTE(Tejas): Rust does not automatically convert non-boolean types to
    // boolean in conditionals, so we cannot do something like this:
    let _number = 3;
    // NOTE(Tejas): we cant do this...
    // if number {
    //     println!("This will not compile because number is not a boolean");
    // }
    println!("------------------------------");


    println!("Loops: ");
    let x = loop {
        println!("Looping...");
        break 42; // you can return a value from a loop using break
    };

    println!("Labeled loops: ");
    'outer: loop {
        println!("Outer loop");
        'inner: loop {
            println!("Inner loop");
            break 'outer; // this will break the outer loop
        }
    }

    println!("Reverse Range: ");
    for number in (1..4).rev() {
        println!("{}, ", number);
    }
}

fn another_function(x: usize) -> usize {
    println!("Another function. {}", x);

    // NOTE(Tejas): The last expression is the return value of the function, and
    // it does not need a semicolon.  if we add a semicolon, it becomes a
    // statement and the function will return the unit type `()`, which is not
    // what we want.
    return x + 1;
}