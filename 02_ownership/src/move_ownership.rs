pub fn run(){
    let s1 = String::from("hello");
    
    // 소유권이 s1 에서 s2 로 이동
    let s2 = s1;
    
    // 컴파일 에러 발생, 유효하지 않음
    // println!("s1 = {}", s1);
    
    println!("s2 = {}", s2);
}