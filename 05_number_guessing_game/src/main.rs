use std::io::{self, Write};
mod game;
mod utils;

fn main() {
    println!("🧠 숫자 맞추기 게임 시작!");

    loop {
        game::play();

        if !ask_retry() {
            println!("게임을 종료합니다.");
            break;
        }

        utils::print_divider()
    }
}

/// 사용자에게 재시작 여부를 묻는 함수
fn ask_retry() -> bool {
    loop {
        print!("게임을 다시 시작하시겠습니까? (yes/no): ");
        io::stdout().flush().expect("출력 실패"); // 출력 버퍼 비우기

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("입력을 읽는 데 실패했습니다.");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => {
                println!("잘못된 입력입니다. 'yes' 또는 'no'를 입력하세요.");
            }
        }
    }
}
