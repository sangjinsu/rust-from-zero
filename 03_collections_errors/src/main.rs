mod option;
mod result;
mod collection;
mod question_mark;

fn main() {
    println!("== Option 실습 ==");
    option::run();

    println!("\n== Result 실습 ==");
    result::run();

    println!("\n== Collection 실습 ==");
    collection::run();

    println!("\n== ? 연산자 실습 ==");
    match question_mark::read_file() {
        Ok(contents) => println!("파일 내용:\n{}", contents),
        Err(e) => println!("에러 발생: {}", e),
    }
}
