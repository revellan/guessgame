use std::io;
use std::io::Write;

fn main() {
    print!("Guess a Number: ");
    io::stdout().flush().expect("Unable to flush stdout");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("You guessed: {}", guess)
}
