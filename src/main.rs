
fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    return s.len();
}

// returns the sliced string
// &str is like a type for string slicking, it can also store string litrals.
fn first_word_better(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    return &s;
}

fn main() {

    // This is valid code but the problem here is that if s gets dropped, the
    // 'word' integer would essentially be containing a meaningless value.

    let mut s = String::from("Hello World!");
    let word = first_word(&s);

    println!("size of first word in {} is {}", s, word);

    // instead of doing operations on strings like above, Rust provides string slicing
    let first_w = &s[0..5];
    let second_w = &s[6..11];

    println!("First Word: {}     Second Word: {} ", first_w, second_w);


    let first_word = first_word_better(&s);

    // Here the best part about &str and is that it points to a slice in String
    // s. It is an immutable reference, but s.clear() needs a mutable reference
    // to s, but Rusts ownership rules state that we cannot have a mutable and
    // an immutable reference to the same memory at the same time
    // s.clear(); // so this will yell at compile time.

    println!("Getting the first word using slicing: {}", first_word);

    // Rust also allows slicing of other types, like integer arrays:
    let a = [1, 2, 3, 4, 5];
    let b = &a[1..4];
    println!("Original: ");
    for i in 0..a.len() {
        print!("{} ", a[i]);
    }
    println!();

    print!("Sliced: ");
    for i in 0..b.len() {
        print!("{} ", b[i]);
    }
    println!();
}