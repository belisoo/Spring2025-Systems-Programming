// Assignment 2: Number Analyzer (09rust-assignments.md)
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,15];
    for i in 0..10 {
        let by_3 = array[i] % 3 == 0;
        let by_5 = array[i] % 5 == 0;        
 //       println!("{}", is_even(array[i]));

        let phrase = match(by_3,by_5){
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => is_even(array[i]).to_string(),
        };

    println!("{}", phrase);       
    }


    let mut n = 0;
    let mut sum: i32 = 0;
    while n < 10 {
        sum += array[n];
        n +=1; 
    }
    println!("The sum of all numbers in the array is {}", sum);

    let mut x: usize = 0;
    let mut largest: i32 = array[0];
    loop {
        if array[x] > largest{
            largest = array[x];
        }
        x += 1;
        if x == 10{
            break;
        }
    }
    println!("The largest number in the array is {}", largest);
}
