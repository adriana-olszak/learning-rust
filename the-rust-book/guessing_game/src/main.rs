use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    game(secret_number);
}

fn game(secret_number: u32) {
    loop {
        println!("Please input your guess.");
        // Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
        // To make a variable mutable, you must add mut in front of the variable name.
        let mut guess = String::new();
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // An associated function is a function that’s implemented on a type, in this case String

        io::stdin()
            // The & indicates that this argument is a reference, which gives you a way to let multiple parts
            // of your code access one piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values,
            // no matter what information they have inside them
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
