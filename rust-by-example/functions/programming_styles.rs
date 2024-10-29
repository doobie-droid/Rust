fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn imperative_style() {
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
}
fn functional_style() {
    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..) //This creates a range of values
        .map(|n| n * n) // This returns an iterator 
        .take_while(|&n_squared| n_squared < upper) // Run the iterator while
        .filter(|&n_squared| is_odd(n_squared)) //Do not take these values
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
fn main() {
    println!("Find the sum of all the numbers with odd squares under 1000");

    imperative_style();
    functional_style();
}
