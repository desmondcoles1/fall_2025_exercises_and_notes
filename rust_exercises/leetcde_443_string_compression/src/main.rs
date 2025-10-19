//Given an array of characters chars, compress it using the following algorithm:
//Begin with an empty string s. For each group of consecutive repeating characters in chars:
//	•	If the group's length is 1, append the character to s.
//	•	Otherwise, append the character followed by the group's length.
//The compressed string s should not be returned separately, but instead, be stored in the input character array chars. 
//Note that group lengths that are 10 or longer will be split into multiple characters in chars.
//After you are done modifying the input array, return the new length of the array.
//You must write an algorithm that uses only constant extra space.

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut v: Vec<char> = Vec::new();
    v.push(chars[0]);
    let l = chars.len();
    for i in 1..l {
        if chars[i] == chars[i-1]{
            //blah, fixed this with copilot, still don't understand the borrowing issue
            // check last element via a temporary copy to avoid holding an
            // immutable borrow while we might mutate `v` below
            if let Some(last) = v.last().copied() {
                if last == '9' {
                    v.push(chars[i]);
                    continue;
                }

                if ('1'..='8').contains(&last) {
                    // mutate the last element in place
                    if let Some(last_mut) = v.last_mut() {
                        let digit = last_mut.to_digit(10).unwrap() + 1;
                        *last_mut = std::char::from_digit(digit, 10).unwrap();
                    }
                } else if last == chars[i] {
                    v.push('1');
                }
            }
        } else {
            v.push(chars[i]);
        }
    }
    chars.append(&mut v);
    let s = chars.len() as i32;
    return s
}

///test

fn main() {
//    let mut chars: &mut Vec<char> = ["a","a","b","b","c","c","c"].to_vec();
//    println!("{:?}", compress(chars))
}


