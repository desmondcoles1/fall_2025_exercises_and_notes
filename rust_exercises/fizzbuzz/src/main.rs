fn main() { //In every program you need a main function. 
    let mut string = String::new(); // empty string

    for i in 1..=117 { //loops over 1 from 117 inclusive
        if i % 3 != 0 && i % 5 !=0 { // check if i is not divisible by 5 or 3
            if !string.is_empty() {
                string.push_str(", "); // add comma if the string is not empty
            }
            string.push_str(&i.to_string()); // append i
        }

        if i % 3 == 0 && i % 5 != 0 { // heck if i is not divisible by 5 but is divisible b3
            string.push_str(", "); 
            string.push_str("fizz"); // append fizz
        }

        if i % 3 != 0 && i % 5 == 0 { 
            string.push_str(", "); 
            string.push_str("buzz"); 
        }

        if i % 3 == 0 && i % 5 == 0 { 
            string.push_str(", ");
            string.push_str("FiZzBuZz");
        }
    }

    println!("{}", string); //print da string
}



