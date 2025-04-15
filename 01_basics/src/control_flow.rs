pub fn run() {
    let score = 85;
    if score >= 90 {
        println!("A")
    } else if score >= 75 {
        println!("B")
    } else {
        println!("C")
    }
    
    for i in 1..5 {
        println!("The value of i is {}", i);
    }
}