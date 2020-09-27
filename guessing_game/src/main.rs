use std::io; // same as using namespace std in cpp
// for input output we need to bring io library into scope
// which comes from standard library

fn main() { // The fn syntax declares a new function
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // In rust, variables are immutable by default
    // String::new() returns the new instance of the string
    // The :: syntax above indicates that new is an associated function
    // of the string type. Some languages call this STATIC method



    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // The stdin function returns an instance of std::io::stdin which is a type
    // that represents a handle to the standard input for your terminal
    // The string argument here needs to be mutable so the method can change the
    // string's content by adding the user input

    println!("You guessed {}", guess);
}