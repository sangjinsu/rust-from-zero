use std::fs::File;

pub fn run() {
    let file = File::open("hello.txt");

    match file {
        Ok(_f) => println!("파일 열기 성공!"),
        Err(e) => println!("에러 발생: {:?}", e),
    }

    let file_result = File::open("nonexistent.txt");

    match file_result {
        Ok(_) => println!("파일을 성공적으로 열었습니다."),
        Err(e) => println!("파일 열기 실패: {:?}", e),
    }

}
