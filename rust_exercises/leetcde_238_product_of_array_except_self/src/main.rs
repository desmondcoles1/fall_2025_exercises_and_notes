pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //initiailizing variables
    let mut total_product = 1; // product of everything in array
    let mut zero_counter = 0; //how many zeros in input
    let mut zero_position = 0; // where is the zero
    let num_nums = nums.len(); //length of relevant vectors
    let mut product_except_self_vector: Vec<i32> = vec![0; num_nums];  //result 
    for i in 0..num_nums{
        if nums[i]== 0{
            zero_counter = zero_counter+1;
            zero_position = i;
            continue
        }
        if zero_counter == 2{
            break
        }
        total_product = total_product * nums[i]
    }
    if zero_counter == 2{
        //do nothing, leave it as 0s
    }
    if zero_counter == 1{
        product_except_self_vector[zero_position] = total_product;
    }
    if  zero_counter == 0{
        for i in 0..num_nums{
            product_except_self_vector[i] = total_product / nums[i] as i32;
        }
    }
    return product_except_self_vector
}

///test

fn main() {
    let nums = [0,1,2,3,4,5,11,0].to_vec();
    println!("{:?}", product_except_self(nums));
}


//this solution was slow and uses division, here it is faster and without division

pub fn product_except_self_better(nums: Vec<i32>) -> Vec<i32> {
    //initiailizing variables
    let num_nums = nums.len(); //length of relevant vectors
    let mut product_except_self_vector: Vec<i32> = vec![1; num_nums];  //result 
    let mut prefix = 1;
    for i in 0..num_nums {
        product_except_self_vector[i] = prefix;
        prefix *= nums[i];
    }
    let mut suffix = 1;
    for i in (0..num_nums).rev() {
        product_except_self_vector[i] *= suffix;
        suffix *= nums[i];
    }
    return product_except_self_vector
}
