extern crate rand; // use rand from dependecies in Cargo.toml

use std::io;
use std::cmp::Ordering; // An enum type, similar to Haskell's
use rand::Rng;          // Rng for "range"

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // `thread_rng()` => thread-local
    // gen_range(lower_inclusive, upper_exclusive)

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();        // Strings are mutable and UTF-8
        // `::f()` =>  type-level function
        io::stdin().read_line(&mut guess)     // `io::stdin()` needs `use std::io;`
            .ok()                             // `&` => reference
            .expect("Failed to read line");   // references are immutable by default

        // `io::stdin()` returns an `io::Result`
        // `ok()` returns an option, i.e. `Some` or `None`
        // `expect()` unwraps an option, calling `panic!` on a `Some`

        // Shadow `guess` to convert it from a `String` to a `u32`
        // trim, parse and cast to 32-bit int
        //
        // If the cast succeeds, bind `guess` to `num`,
        // otherwise if it results in an error,
        // ignore the error and restart the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue, // Restart the loop
        };

        println!("You guessed: {}", guess);    // `{}` => placeholder
        // e.g. `println!("x and y: {} and {}", x, y);`

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;                // Exit the loop
            }
        }
    }
}
