fn main() {
    printlin!("Guess the fucking number!");
    printlin!("Please input your guess.");

    let mut guess = Striong::new();

    io:stdin()
        .read_line(&mut guess) //mutable. default is immutable
        .expect("Failed to read line");
    printlin!("You Guess: {guess}");
}
