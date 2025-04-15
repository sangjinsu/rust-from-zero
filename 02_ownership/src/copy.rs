pub fn run() {
    let x = 42;
    let y = x; // 복사 발생 (Copy 트레이트 덕분)
    
    println!("x = {}, y = {}", x, y);
}