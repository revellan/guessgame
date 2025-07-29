use rand::Rng;
use std::{cmp::Ordering, io, io::Write};

fn main() {
    let mut attempts: u8 = 1;
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        if attempts >= 255 {
            println!(
                "\n\nYou lost!\nYou could have just guessed everything from 0..100 and it would have taken you less attempts ;)"
            );
            break;
        }
        let guess: i8 = loop {
            let guess_str = get_user_num();
            if let Ok(nm) = guess_str.trim().parse() {
                if nm <= 100 && nm >= 0 {
                    break nm;
                }
            };
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => attempts = repeat("Too small!", attempts),
            Ordering::Greater => attempts = repeat("Too big!", attempts),
            Ordering::Equal => {
                let attempt_str = ["first", "second", "third"];
                if attempts <= 3 {
                    println!(
                        "Congratulations! You guessed correctly on the {} attempt!",
                        attempt_str[(attempts - 1) as usize]
                    );
                } else {
                    println!(
                        "Congratulations! You guessed correctly on the {}th attempt!",
                        attempts
                    );
                }
                break;
            }
        }
    }
}
fn repeat(out: &str, num: u8) -> u8 {
    println!("{}", out);
    return num + 1;
}
fn get_user_num() -> String {
    print!("Guess a Number: ");
    io::stdout().flush().expect("Unable to flush stdout");
    let mut return_value = String::new();
    io::stdin()
        .read_line(&mut return_value)
        .expect("Failed to read line!");
    return return_value;
}
