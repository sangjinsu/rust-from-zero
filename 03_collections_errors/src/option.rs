pub fn run() {
    let maybe_num = Some(10);

    match maybe_num {
        Some(n) => println!("숫자: {}", n),
        None => println!("값이 없습니다."),
    }

    let none_val: Option<i32> = None;
    println!("None 값: {:?}", none_val);

    let maybe_name: Option<&str> = Some("Jinsu");

    match maybe_name {
        Some(name) => println!("안녕하세요, {}님!", name),
        None => println!("이름을 알 수 없습니다."),
    }

    // if let 문법
    if let Some(name) = maybe_name {
        println!("환영합니다, {}!", name);
    }
}
