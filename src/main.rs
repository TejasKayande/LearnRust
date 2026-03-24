
// We can store data of type reference in structs that refere to object outside
// the struct but for that we need to specify the object lifetime for that..
// Thats why we kept username and email as Strings for now instead of &str.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structure
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// A Struct with no fields
// Unit-like structs can be useful when you need to implement a trait on some
// type but don’t have any data that you want to store in the type itself
struct UnitStruct;

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("tejas"),
        email: String::from("tejas@mail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("another@mail.com"),
        // This fillls the rest of the values. This should come at the very last
        ..user1 
    };

    user1.email = String::from("Hello World!");

    let c= Color(18, 18, 18);
    let p= Point(55, 100, 82);

    // destructing a point. Just like a tuple.
    let Point(p, q, r) = p;
}