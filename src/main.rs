use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let mut attempts: u32 = 0;
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let guess = get_user_num();
        let res = compare(&guess, &secret_number);
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
fn get_user_num() -> i32 {
    print!("Guess a Number: ");
    io::stdout().flush().expect("Unable to flush stdout");
    let mut return_value = String::new();
    io::stdin()
        .read_line(&mut return_value)
        .expect("Failed to read line!");
    let return_value: i32 = return_value.trim().parse().expect("Enter a valid Integer!");
    return return_value;
}
