
fn main() -> () {


    // Ownership is a set of rules that govern how a Rust program manages
    // memory. All programs have to manage the way they use a computer’s memory
    // while running. Rust uses a third approach: Memory is managed through a
    // system of ownership with a set of rules that the compiler checks. If any
    // of the rules are violated, the program won’t compile. None of the
    // features of ownership will slow down your program while it’s running.

    // Ownership rules:
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // allocation memory on the heap at runtime
        // here s owns the string "hello".
        let mut s = String::from("hello");

        // ownership of the string is moved to s2, s is no longer valid.
        // This is a Shallow Copy
        // let s2 = s;

        // ownership of the string is cloned to s2, s is still valid.
        // This is a Deep Copy
        let mut s2 = s.clone(); 
        s2.push_str("world");
        
        // because s was assigned a new value, Rust called drop() on the old s
        // and a new string was allocated on the heap for the new value.
        // This happens in order: first it creates a new string on the heap,
        // then it checks that the old s is no loger pointed by anything and
        // therfore is out of scope, so Rust calls drop().
        s = String::from("test");

        println!("s: {}, s2: {}", s, s2);

    } // s goes out of scope and memory is freed. Standard RAII pattern.

    {
        let s = String::from("hello");
        // takes_ownership(s);
        let s = take_and_give_ownership(s);

        println!("s: {}", s);
    }

    {
        let s = String::from("hello");
        let (s, len) = calculate_length(s);

        println!("The length of '{}' is {}.", s, len);
    }

    // All of the ownership rules only apply to values that are stored on the heap.
    // Values that are stored on the stack have a known size at compile time and
    // are stored directly on the stack. They can be copied by simply copying
    // their bits, which is very fast.

}

fn _takes_ownership(some_string: String) {

    println!("This string will now go out of scope: {}", some_string);
} // some_string goes out of scope and memory is freed.

fn take_and_give_ownership(some_string: String) -> String {

    return some_string; // some_string is moved out to the caller.
}

fn calculate_length(s: String) -> (String, usize) {

    let length = s.len();

    // s is moved out to the caller before it goes out of scope.
    return (s, length); // returning multiple values usign tuples.
}