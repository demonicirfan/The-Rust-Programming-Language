use rand::Rng;
use std::io;

fn main() {
    // a macro that prints a string to the screen
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is: {}", secret_number);

    println!("Please input your guess!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);
}
