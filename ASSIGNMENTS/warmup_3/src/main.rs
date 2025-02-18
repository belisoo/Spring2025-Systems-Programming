// Problem #3: Mutable Reference Sum
// Write a modified sum function that takes a mutable reference for the destination of the sum from low to high.

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total = *total + i;
    }
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let low: i32 = 0;
    let high: i32 = 100;
    // total should be 5050
    let mut total: i32 = 0;
    sum(&mut total, low, high);
    println!("The total is: {}", total) 
}