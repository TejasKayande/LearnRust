use std::f32::consts::E;

enum Message {
    Quit(u8), // u8 is the window id that needs to be quit.
    Move,
    Jump,
    Sneak,
    Sprint
}

fn handle_quit(msg: Message) -> String {

    // we can use let else to match a single pattern and return early if it does not match.
    // if the pattern matches, the value is assigned to the variable and we can
    // use it in the rest of the function.

    let Message::Quit(window_id) = msg else {
        return String::from("The message was not Quit!");
    };

    // here because we did not return in the else block,
    // we can use window_id in this scope.

    format!("The window with ID {} was quit!", window_id)
}

fn main() -> () {

    // Very safe way to have NULL values.

    let opt: Option<usize> = Option::None;
    // let opt: Option<usize> = Option::Some(5);
    // let opt: Option<usize> = Option::Some(15);

    let sum: String = match opt {
        Option::None             => String::from("The Value is None"),
        Option::Some(val) => format!("{}", val)
    };

    println!("{}", sum);

    // match needs to have all the possible values coverd.
    let number = 9;

    let was_three = match number {

        3 => true,
        4 => false,
        _other => false, // other is a value that number was, that did not match an cases.

        // if we dont want to use the variable, we can use just an '_' to catch the default value.
        _ => false
    };

    println!("{}", was_three);


    // to match only a single pattern in we can use if let

    if let Some(5) = opt {
        println!("The number in Option<T> was 5");
    } else if let None = opt {
        println!("The number in Option<T> was None ");
    } else {
        println!("The number in Option<T> was not 5 or None");
    }

    let msg = Message::Quit(5);
    let result = handle_quit(msg);
    println!("{}", result);

}