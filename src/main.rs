extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game");
    println!("=====================================\n");

    println!("Guess a number between 1 and 100 (inclusive) - type 'quit' to give up");
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        //  Get user's guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Exit game
        if guess.to_lowercase().trim() == "quit" {
            println!("\nThe number was {}, thanks for playing - quitter", number);
            break;
        }

        //  Convert user input to unsigned int
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("You must type a number!");
                continue;
            }
        };

        //  Check guess by pattern matching
        match guess.cmp(&number) {
            Ordering::Less => println!("\nYour guess of {} was too small, try again", guess),
            Ordering::Greater => println!("\nYour guess of {} was too large, try again.", guess),
            Ordering::Equal => {
                println!("\nYou got it!");
                break;
            }
        }
    }
}
