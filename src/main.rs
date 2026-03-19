use rand::Rng;

fn main() {

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number: ");

        let mut guess= String::new();

        std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Your Guess was not a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&random_number) {

            std::cmp::Ordering::Less => {
                println!("Too small!");
            },

            std::cmp::Ordering::Greater => {
                println!("Too big!");
            },

            std::cmp::Ordering::Equal => {
                println!("You win! The Number was: {}", random_number);
                break;
            },
        }
    }
}
