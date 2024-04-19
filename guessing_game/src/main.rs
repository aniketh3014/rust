use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welocme to the guessing game!");
    loop {
        println!("Now enter the number you have guessed: ");

        let random_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("There was an error reading the input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you have guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Exactly that!");
                break;
            }
        }
    }
}