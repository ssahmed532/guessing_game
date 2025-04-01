use rand::Rng;
use std::cmp::Ordering;
use std::io;

const QUIT_CMD: &str = "quit";
const GUESS_LOW_VALUE: i32 = 1;
const GUESS_HIGH_VALUE: i32 = 100;

fn main() {
    println!("Welcome to the \'Guess the number!\' game!!");

    loop {
        println!(
            "Please input your guess - a valid, non-negative integer number (between 1 and 100):"
        );
        println!("(OR enter \'quit\' to exit from this game)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        if input.to_lowercase() == QUIT_CMD {
            break;
        }

        let number_guess: i32 = input.parse().expect("Failed to parse string to integer");
        if number_guess < GUESS_LOW_VALUE || number_guess > GUESS_HIGH_VALUE {
            println!("You entered a number ({}) that's outside the valid range for this game - please try again!", number_guess);
            continue;
        }

        let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
        // println!("My secret number is: {}", secret_number);
        println!("You guessed: {}", number_guess);

        // TODO:
        // - track the total number of attempts/guesses made by the user
        //   and print when the user successfully guesses the secret number
        // - if the user's guess is within 5-10 vals of the secret number,
        //   change the msg displayed to indicate how close the user's guess was!!!

        match number_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations... You win!");
                break;
            }
        }
        println!();
        println!();
    }
}
