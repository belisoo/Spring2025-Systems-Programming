 fn main() {
//     let x = 5;

//     let rem = x%3;

//     if rem == 0 {
//         println!("Divisible");
//     }
//     else if rem == 1 {
//         println!("Remainder = {}", rem);
//     }
// -------------------------------------------------------
    // let x = 5;
    // let rem = x %3;

    // let phrase = match rem {
    //     0 => "Remainder is Zero",
    //     1 => "Remainder is One",
    //     2 => {
    //         println!{"This was amazing choice!"};
    //         "Remainder is Two",
    //     },
    //     _ => "#", //println!(),
    // };

    // println!("{}", phrase);
// -----------------------------------------------------------
        let number = 5;
        
        // let divisible_by_two = if number % 2 == 0 {
        //     "even" // no semicolon, because then it becomes an expression
        // } else {
        //     "odd" // both arms or branches should evaluate to the same type
        // };
        
        let mut divisible_by_two = "odd";
        if number %2 == 0 {
            divisible_by_two = "even";
        }
        
        println!("Number {} is a type of {}", number, divisible_by_two);
}
