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
    v.push(chars[0])
    let l = chars.len();
    for i in 1..l {
        if chars[i] = chars[i-1]{
            if v.last() == 9 {
                v.push(chars[i])
            }
            if (1..9).contains(&v.last().unwrap()) {
                v.last() = v.last()+1
            }
            if if v.last() == chars[i] {
                v.push(1)
            }
        }
        else {
            v.push()
        }
    }
    s.append(&mut v);
    return s.len()
}

///test

//fn main() {
//    let mut chars: &mut Vec<char> = ["a","a","b","b","c","c","c"].to_vec();
//    println!("{:?}", compress(chars))
//}


