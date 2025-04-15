fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_with_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| x * 2);

    let replaced = process_vector(numbers, |x| {
        if x > 2 {
            0
        } else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);   // Output: [2, 4, 6]
    println!("Replaced: {:?}", replaced); // Output: [1, 2, 0]


/*    let doubled = process_vector_with_loop(numbers.clone(), |x| x * 2);

    let replaced = process_vector_with_loop(numbers, |x| {
        if x > 2 {
            0
        } else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);   // Output: [2, 4, 6]
    println!("Replaced: {:?}", replaced); // Output: [1, 2, 0] */
} 