use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number between 1 and 100
    // println!("The secret number is: {secret_number}"); //print the secret number
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //create var with mutable string instance

        io::stdin() //call stdin fn from io module
            .read_line(&mut guess) // read ip to guess var, pass by reference
            .expect("Failed to read line"); //handle potential error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //convert string to u32
            Err(_) => continue, //continue to next iteration if parse fails
        };

        println!("You guessed: {guess}"); 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}