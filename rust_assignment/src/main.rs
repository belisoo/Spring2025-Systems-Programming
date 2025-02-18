fn main() {
    let word = "UTRGV".to_string();
    let borrow_word = &word;
   
    let borrow_word1 = &word;
   
    print_string(&word);
}

fn print_string(w: &String) {
    println!("{}",w);
}