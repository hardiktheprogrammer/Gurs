use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0_u32, 100_u32);
    println!("Please Input your Guess:{}", secret_number);
    loop {
        println!("Please Input Your Guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("please Type a  number!"); // guess
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Less => println!("Too Big!"),
            Ordering::Greater => println!("You win!"),
            println!("You win");
            break;
        }
    }

    // let secret_number: i32
}
