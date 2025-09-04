//This code will check if a string is a palindrome.

use std::io; // this brings the "io" functionality into my scope, as in "io" is a functionality that is defined in the standard module, I can bring it into my scope so now I can just write  "io::" in my code instead of "std:io::"

fn main() {
    println!("Give me a string and I will tell you if it is a palindrome."); //prints text
    let mut string = String::new(); // this creates a mutable empty string

    io::stdin()
        .read_line(&mut string)     // read input into the String
        .expect("Error while reading line.");
    let string = string.trim(); // This removes a newline character that is automatically added by the read_line command
    let reversed: String = string.chars().rev().collect(); //reverses my string

    if reversed == string  {
        println!("The string {} is a palindrome.", string);
    } else {
        println!("The string {} is not a palindrome.", string);
    }


}
