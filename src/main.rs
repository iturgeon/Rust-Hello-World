extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess a number!");

    loop {

        println!("Input a guess.");

        let guess :u32 = get_input();

        if guess == 0 {
            println!("Guesses must be positive integers");
            continue;
        }

        println!("You Guessed: {}, the secret is {}", guess, secret);


        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("You WIN");
                break;
            },
        }
    }
}

fn get_input() -> u32 {

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_)  => 0
    };

    return guess;
}
