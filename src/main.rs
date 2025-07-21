use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let mut attempts: u32 = 1;
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess: i32;
        loop {
            let guess_str = get_user_num();
            match guess_str.trim().parse() {
                Ok(num) => {
                    guess = num;
                    break;
                }
                Err(_) => continue,
            }
        }
        let res = compare(&mut guess, &secret_number);
        if res == 1 {
            attempts = repeat("Too small!", attempts);
        } else if res == 2 {
            attempts = repeat("Too big!", attempts);
        } else {
            println!(
                "Congragulations! You guessed correctly after {} attempts!",
                attempts
            );
            break;
        }
    }
}
fn repeat(out: &str, num: u32) -> u32 {
    println!("{}", out);
    return num + 1;
}
fn compare(guessed_num: &i32, secret_num: &i32) -> i32 {
    match guessed_num.cmp(&secret_num) {
        Ordering::Equal => return 0,
        Ordering::Less => return 1,
        Ordering::Greater => return 2,
    }
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
