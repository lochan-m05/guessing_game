use std::{intrinsics::const_eval_select, io};
use rand::{rng, Rng};

fn main() {
    println!("++++++++++++++++++++++");
    println!("Welcome to Guessing Game !!");
    println!("++++++++++++++++++++++");

    
    println!("Enter Your Name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the name");

    
    println!("Hello {}, welcome to a rusty guessing game!!", name.trim());
    println!("Now, please enter your guess:");

  
    let mut guess = u128::;
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the guess");

    // You can now use the guess
    println!("You guessed: {}", guess.trim());


      // Random number gen
    
    let randomnumber = rand::thread_rng().gen_range(1..101);
    if randomnumber == guess {
          println!("Your correct!!");
    }
  
   
    println!("Random number:{}",randomnumber);
    

}
