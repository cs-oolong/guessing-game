use std::io; // import
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!"); // macro
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // rng local to the execution thread, seeded by OS, bounds are inclusive on both sides

    loop {
        println!("Please input your guess.");
        // let apples = 5; -> immutable variable by default
        let mut guess = String::new(); // make guess mutable
        // String::new -> :: means new is a an *associated function*, it's implemented on a type

        io::stdin().read_line(&mut guess).expect("Failed to read line"); // expect will make the program crash in case there is an error
        // & = reference, avoid unnecessary copies
        // & mut = mark the reference as mutable, because by default references are also immutable
    
        // enum is a type that can be in one of multiple possible states. We call each possible state a variant.
        // Result type - to encode error-handling information.
    
        // a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.
    
        // Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
    
        // Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.
    
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example. 
    
        let guess: u32 = guess.trim().parse().expect("Please type a number");
        // : = annotating variable type
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => println!("you win"),
        }
    }
}