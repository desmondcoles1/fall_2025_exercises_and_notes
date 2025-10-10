fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let l_1 = str1.len();
        let l_2 = str2.len();
        let l = gcd(l_1 as i32, l_2 as i32);
        let str1_l= &str1[..l as usize];
        let str2_l = &str2[..l as usize];
        let mut gcdstr = String::new();
        if str1_l == str2_l && str1 == str1_l.repeat(l_1 / l as usize) && str2 == str1_l.repeat(l_2 / l as usize)
        {
            gcdstr = String::from(str1_l);
        }
        else
        {
            gcdstr = String::from("");
        }
        return gcdstr;
    }



fn main() {
    let str_1 = String::from("CEET");
    let str_2 = String::from("CODE");
    let result = gcd_of_strings(str_1, str_2);
    println!("The GCD of the two strings is: {}", result);
}

