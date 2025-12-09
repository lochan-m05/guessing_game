use std::io;
use std::cmp::Ordering;
use rand::Rng;

const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 1000;

fn main() {
    println!("Guess the number between {} and {}!", MIN_RANGE, MAX_RANGE);

    let secret_number = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please type a number.");
                continue;
            }
        };

     
        if guess < MIN_RANGE || guess > MAX_RANGE {
            println!("Out of bounds! Stick to {}-{}", MIN_RANGE, MAX_RANGE);
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}
