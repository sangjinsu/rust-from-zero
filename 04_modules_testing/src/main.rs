mod calculator;
mod utils;

use crate::calculator::add;

fn main() {
    let sum = calculator::add(5, 10);
    let diff = calculator::subtract(10, 5);
    
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    
    let result = add(5, 10);
    println!("Result: {}", result);
}
