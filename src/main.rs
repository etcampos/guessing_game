use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("The winning number will be a number between 0 and 100!");

    //println!("The secret number is: {secret_num}"); --this is for testing 

    loop {
        println!("Please enter your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
