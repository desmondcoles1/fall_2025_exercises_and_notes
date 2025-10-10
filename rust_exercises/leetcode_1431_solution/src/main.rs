
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut will_have_greatest: Vec<bool> = vec![false; candies.len()];
    let candies_max = *candies.iter().max().unwrap();
    for i in 0..candies.len(){
        if candies[i] + extra_candies >= candies_max{
            will_have_greatest[i] = true;
        }
    }
    return will_have_greatest
}

fn main() {
    let candies: Vec<i32> = [2,3,5,1,3].to_vec();
    let extraCandies: i32 = 3;
    println!("{:?}", kids_with_candies(candies, extraCandies))
}