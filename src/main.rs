use std::io;
//input/output library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // we create a mutable variable guess of string type

    // we can als use std::io::stdin even if we used the 1st line std::io
    // he stdin function returns an instance of std::io::Stdin, 
    // which is a type that represents a handle to the standard input for your terminal
    io::stdin()
        .read_line(&mut guess)
        // take whatever the user types into standard input and append that into a string 
        // without overwriting its contents 
        // so we therefore pass that string as an argument. 
        // The string argument needs to be mutable so the method can change the string’s content.
        // & indicates that this argument is a reference
        // multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

