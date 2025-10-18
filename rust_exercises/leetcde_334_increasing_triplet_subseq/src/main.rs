// Given an integer array nums, return true if there exists a triple of indices 
//(i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. 
//If no such indices exists, return false.

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut result = false;
    if nums.len() <= 2 {
        result = false
    }
    else{
        for i in 1..nums.len()-1 {
            let prefix = &nums[0..i];
            let suffix = &nums[i+1..nums.len()];  
            if *prefix.iter().min().unwrap() < nums[i] 
                &&  nums[i]< *suffix.iter().max().unwrap() 
                    {
                result = true;
                break
            }
        }
    }
    return result
}

//this solution O(n^2), it is too slow, the following O(n) and works better

pub fn increasing_triplet_better(nums: Vec<i32>) -> bool {
    let mut first = i32::MAX;
    let mut second = i32::MAX;

    for &n in &nums {
        if n <= first {
            first = n;              // smallest so far
        } else if n <= second {
            second = n;             // second smallest so far
        } else {
            // found n > second > first
            return true;
        }
    }
    false
}

///test

fn main() {
    let nums = [10000,1,12,3,4,5,0,0,0].to_vec();
    println!("{:?}", increasing_triplet(nums))
}


