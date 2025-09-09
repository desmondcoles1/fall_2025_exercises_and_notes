//This is my solution to Leetcode 151. Here is the problem statement. 
//Given an input string s, reverse the order of the words.

//A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

//Return a string of the words in reverse order concatenated by a single space.

//Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

//solution
//Note: my solution does not handle general strings in rust, because Rust strings can take UTF-8 characters which may occupy more than one byte, here .get() may slice things in the wrong place for non-ASCII  characters
//lol the runtime is like laughably bad compared to other solutions


pub fn reverse_words(s: String) -> String {
    let ss = " ".to_string() + &s;
    let mut reversed_string = String::new();
    let length_of_string = ss.chars().count();
    let mut dummy_word = String::new();

    for b in 1..length_of_string {
        let c = ss.chars().nth(b).unwrap();
        let next = ss.chars().nth(b + 1).unwrap_or(' '); 
        let prev = ss.chars().nth(b - 1).unwrap_or(' '); 
        if c == ' ' { 
            {} //pass
        }
        if c != ' ' && next != ' ' && prev == ' '{ //found a word
            dummy_word.clear();
            dummy_word.push(c);
        }
        if c != ' ' && next != ' ' && prev!= ' '{ //inside a word
            dummy_word.push(c);
        }       
        if c != ' ' && next == ' '  && prev!= ' ' { //word has ended
            dummy_word.push(c);
            if !reversed_string.is_empty(){
                dummy_word.push(' ')
            }
            reversed_string.insert_str(0, &dummy_word);
            dummy_word.clear();
        }
        if c != ' ' && next == ' ' && prev == ' '{ //found a letter
            dummy_word.clear();
            dummy_word.push(c);
            if !reversed_string.is_empty(){
                dummy_word.push(' ')
            }
            reversed_string.insert_str(0, &dummy_word);
            dummy_word.clear();
        }
    }
    return reversed_string
}

fn main() {
    let s = String::from("F R  I   E    N     D      S      ");
    let reversed =reverse_words(s);
    println!("{}", reversed); 
}