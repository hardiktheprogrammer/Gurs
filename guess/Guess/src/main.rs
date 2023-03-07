use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..-100);
    println!("Please Input your Guess:{}", secret_number);
    println!("Please Input Your Guess.");

    // let secret_number: i32
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
