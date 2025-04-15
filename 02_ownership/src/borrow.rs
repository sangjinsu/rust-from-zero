fn print_string(s: &String) {
    println!("(read only): {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" + changed!");
}

pub fn run() {
    let s = String::from("hello");
    print_string(&s); // 불변 참조 
    print_string(&s); // 여러 번 가능
    
    let mut s2 = String::from("mutable hello");
    change_string(&mut s2); // 가변 참조는 한 번만
    
    println!("after mutation s2 = {}", s2);
}