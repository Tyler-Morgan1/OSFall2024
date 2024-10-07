
fn is_even(i: i32) -> bool {
    if i % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {

    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    
    let mut x = false;
    let mut y = 0;
    
    for idx in 0..nums.len() {
        y = nums[idx];
        // print whether even or odd
        x = is_even(y);
        println!("{}", x);

        
        //FizzBuzz implimentation
        if y % 3 == 0 && y % 5 == 0 {
            println!("FizzBuzz");
        } else if y % 3 == 0 {
            println!("Fizz");
        } else if y % 5 == 0 {
            println!("Buzz");
        }
    }
}
