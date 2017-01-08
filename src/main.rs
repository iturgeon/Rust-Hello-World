extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut guesses: Vec<u32> = vec![];

    println!("Guess a number!");

    loop {

        println!("Input a guess.");

        let guess :u32 = get_input();

        if guess == 0 {
            println!("Guesses must be positive integers");
            continue;
        }

        guesses.push(guess);

        println!("You Guessed: {}, the secret is {}", guess, secret);


        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("You WIN");
                print_guesses(guesses);
                break;
            },
        }
    }
}

fn print_guesses(guesses :Vec<u32>) {
    println!("Here are your {} guesses", guesses.len());
    for f in &guesses {
        println!("{}", f);
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
