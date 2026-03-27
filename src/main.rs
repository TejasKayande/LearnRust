
enum IpAddrKind1 {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAddrKind1,
    address: String,
}

enum IpAddrKind2 {
    // Store the value directly inside the enum option
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32)
}

// adding methods on Message.
impl Message {
    // handling Message type this way...
    fn update(&self) -> String {
        match self {
            Message::Quit                                   => "Quittin".to_string(),
            Message::Move {x, y}                => format!("Moving to {} {}", x, y),
            Message::Write(st)                     => format!("Writing: {}", st),
            Message::ChangeColor(r, g, b) => format!("Change Color: {} {} {}", r, g, b)
        }
    }
}

fn main() -> () {

    // Instead of doing it this way, Rust provides a way to store values inside
    // ENUM option
    let _local: IpAddr1 = IpAddr1 { 
        kind: IpAddrKind1::V4, 
        address: String::from("192.168.0.1")
    };

    let my_local_ip = IpAddrKind2::V4(String::from("192.168.20.12"));
    if let IpAddrKind2::V4(ip) = my_local_ip {
        println!("{}", ip);
    }

    let msg = Message::Quit;
    println!("{}", msg.update());

    // There is no NULL in Rust, but to accomodate this they have an enum in the std lib
    // Option<T> {
    //     Node,
    //     Some(T)
    // };

    // now we have the ability to set y to None, which essesntially means NULL
    let y: Option<i8> = Some(5);

    // In other words, you have to convert an Option<T> to a T before you can perform T
    // operations with it. Generally, this helps catch one of the most common issues
    // with null: assuming that something isn’t null when it actually is.
    let c = 5 + y; // cant do integer + Option<i8>
}