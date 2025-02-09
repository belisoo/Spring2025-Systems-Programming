// Assignment 3: Guessing Game (09rust-assignments.md)
fn check_guess(guess: i32, secret: i32) -> i32 {
    let too_high = (guess > secret) == true;
    let too_low = (guess < secret) == true;

    let answer: i32 = match(too_high,too_low){
        (true, false) => 1,
        (false, true) => -1,
        (false, false) => 0,
        (true, true) => todo!(),
    };
    return answer;
}

fn main() {
    let mut secret: i32 = 5;
    let mut count: i32 = 0;
    let mut input: i32 = 9;    
    loop {
        count += 1;
        
        if check_guess(input, secret) == 0 {
            println!("Your guess was correct!"); 
            break;
        }
        else if check_guess(input, secret) == 1 {
            println!("Your guess was too high."); 
            input -=1;
        }
        else {
            println!("Your guess was too low."); 
            input += 1;       
        }

    }
    println!("It took you {} guesses", count);
}
