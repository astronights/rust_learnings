use std::io;
use std::io::Write;

use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Number Guessing Game!");
    // println!("The Secret Number is {}", secret_num);

    loop {
        print!("Guess a number between 1 - 100 (Inclusive): ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read your guess :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Number out of range: 1 - 100");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Please guess a valid number");
                continue;
            },
        };

        println!("Your Guess: {guess}");

        let scale = 5;

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small {}!", "*".repeat(((secret_num - guess)/scale).try_into().unwrap())),
            Ordering::Greater => println!("Too Big {}!", "*".repeat(((guess - secret_num)/scale).try_into().unwrap())),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
