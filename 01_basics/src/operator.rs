pub fn run() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tuple;
    println!("The value of x is {}", x);
    
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is {}", arr[0]);
}