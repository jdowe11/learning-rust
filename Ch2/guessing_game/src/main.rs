use std::io; // standard IO library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop { // loop to allow for continuous guessing
        println!("Please input your guess.");

        // let creates a new variable, type inference 
        let mut guess = String::new(); // by default, all variables are immutable
        // must specifically say if it can be changed or not

        // :: always implies that the object to the right is a function of the thing to the left
        io::stdin() // if we never used std::io, could also say std::io::stdin()
            .read_line(&mut guess) // by default, all references (&) are immutable
            .expect("Failed to read line"); // standard is to use newlines

        // shadow guessed variable, make it IMMUTABLE, and ensure it has the right type
        let guess: u32 = match guess.trim().parse() { // trim is a String function
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed {guess}");


        // Pattern Matching to Guess number
        match guess.cmp(&secret_number) { // pattern matching similar to OCaml
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { // exiting loop if guessed
                println!("You win!");
                break;
            }
        }
    }
}
