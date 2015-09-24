use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();        // Strings are mutable and UTF-8
                                          // ::f() =>  type-level function
    io::stdin().read_line(&mut guess)     // `io::stdin()` needs `use std::io;`
        .ok()                             // & => reference
        .expect("Failed to read line");   // references are immutable by default

    // `io::stdin()` returns an `io::Result`
    // `ok()` returns an option, i.e. `Some` or `None`
    // `expect()` unwraps an option, calling `panic!` on a `Some`

    println!("You guessed: {}", guess);   // `{}` => placeholder
    // e.g. `println!("x and y: {} and {}", x, y);`
}
