// Rust book chapter 2: Guessing Game 

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let range: [u32;2] = [1,100];
    println!("Guess the number in range {} - {}!", range[0], range[1]);
        
    
    let secret_number = rand::thread_rng().gen_range(range[0], range[1] + 1);
    //println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
