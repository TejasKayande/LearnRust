
fn main() -> () {

    let _v1: Vec<i32> = Vec::new();

    // macro to create a vector with initial values
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);

    // adding values using push method
    v.push(4);
    v.push(5);
    v.push(-5);

    println!("{:?}", v);

    let val = &v[4]; // this will panic at runtime if index is out of bounds.
    println!("val element is {}", val);

    // to fix out of bounds error, we can return Option type using get method.
    let index = 10;
    let val = v.get(index); // this will return None if index is out of bounds.
    match val {
        Some(value) => println!("fifth element is {}", value),
        None => println!("There is no {}th element.", index),
    }

    v[2] = 100; // this works
    println!("{:?}", v);

    let x: &mut i32 = &mut v[2]; // this also works, but we cannot use v[2] until we are done with x.

    // v[2] = 200; // this will throw error because we have a mutable reference to v[2] in x.

    *x = 1000; 

    println!("{:?}", v);


    // here we have are holding reference to v[2] in x, so we cannot add new element to v until we are done with x.
    v.push(10); // this will throw error because we cannot borrow v as mutable

    // once we try to do any operations on v, we are done with x, so we can use v again.
    // compiler will throw erro if we try to use this reference after we are done with it.
    // *x = 2000; 

    println!("{:?}", v);


    // iterating over vector
    for i in &v {
        println!("{}", i);
    }

    // vectors only hold one type of data, but we can use enum to get around this...
    enum SpreadSheetCell {
        Roll(i32),
        Name(String),
        Grade(String),
    }

    let _row = vec![
        SpreadSheetCell::Roll(3),
        SpreadSheetCell::Name(String::from("Alice")),
        SpreadSheetCell::Grade(String::from("A+")),
    ];

    // when a vector is dropped, all of its elements are also dropped.
    // The borrow checker ensures we dontt have any references to elements of a
    // vector after the vector is dropped.

}
