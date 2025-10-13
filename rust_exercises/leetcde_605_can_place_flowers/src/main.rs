
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut plant_count= 0;
    let mut flowerbed_with_new_plants: Vec<i32> = vec![0; flowerbed.len()+2];
    println!("{:?}", flowerbed_with_new_plants);
    for i in 0..flowerbed.len(){
        flowerbed_with_new_plants[i+1]=flowerbed[i]
    }//test this
    println!("{:?}", plant_count);
    for i in 1..flowerbed.len()+1{
        println!("{:?}", flowerbed_with_new_plants);
        println!("{:?}", plant_count);
        if flowerbed_with_new_plants[i]==0 && flowerbed_with_new_plants[i-1]==0 && flowerbed_with_new_plants[i+1]==0{
            flowerbed_with_new_plants[i]=1;
            plant_count = plant_count+1
        }
        println!("{:?}", flowerbed_with_new_plants);
        println!("{:?}", plant_count);
    }//test this
    if plant_count >= n{
        return true
    }
    else {
        return false
    }
}

///this feels like such a slow solution!


//////testing


fn main() {
    let flowerbed = [1,0,0,0,1].to_vec();
    let n = 1;
    println!("{:?}", can_place_flowers(flowerbed, n));
}