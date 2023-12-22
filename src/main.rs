use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the fucking number!"); // adds new line
    let secret_number = rand::thread_rng().gen_range(1..=100);
    print!("Please input your guess: "); // doesnt add new line

    io::stdout().flush().expect("Failed to flush stdout"); // Flush the buffer

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //mutable. default is immutable
        .expect("Failed to read line");
    println!("You Guess: {}", guess.trim());

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!!")

    }
}
