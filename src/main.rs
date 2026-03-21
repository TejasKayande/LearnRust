// The Rules of References

// Let’s recap what we’ve discussed about references:

//     At any given time, you can have either one mutable reference or any
//     number of immutable references.  
//     References must always be valid. You cannot have dangling references.

fn main() -> () {

    let mut s1 = String::from("hello");

    // This safe guard is only for mutable references.
    // we can have multiple immutable references.
    // and the compiler will only throw if we use mutable r1 after we create r2.
    // If we never use r1 after we create r2, then the code is perfectly valid.
    let _r1  = &mut s1;
    let _r2  = &mut s1;

    // This will break if we try to use r1 and r2
    // println!("{} and {}", _r1, _r2);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("The value of s1 is '{}'.", s1);
}

// NOTE(Tejas): here because s is a reference to a String, the ownership is
// still with the formal parameter s1. so we dont need to return the ownership
// back.
fn calculate_length(s: &String) -> usize {

    return s.len();
}

// NOTE(Tejas): here we are passing a mutable reference to the function change,
// so that we can modify the value of the string that is being passed in. if we
// had passed an immutable reference, we would not be able to modify the value
// of the string.
fn change(some_string: &mut String) {

    // Mutable references have one big restriction: If you have a mutable
    // reference to a value, you can have no other references to that value.
    // This code that attempts to create two mutable references to s will fail:

    some_string.push_str(", world");
}


fn dangling_reference() -> &String {

    let s = String::from("Test");

    // This will not work because rust ensures that the data will not be dropped
    // before reference to that data.
    return &s;

    // Instead of this we should just return the ownership of the string with
    // return s;
}