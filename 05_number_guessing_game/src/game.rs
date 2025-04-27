use rand::Rng;
use std::io::{self, Write};

pub fn play() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut turns = 0;
    let max_turns = 3;

    println!("🎯 숫자 맞추기 게임 시작!");
    println!("목표: {}번 안에 맞추세요!", max_turns);

    loop {
        turns += 1;
        println!("\n[Turn {}/{}]", turns, max_turns);
        print!("1~100 사이 숫자를 입력하세요: ");
        io::stdout().flush().expect("출력 실패");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("입력을 읽는 데 실패했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ 숫자를 입력하세요!");
                continue;
            }
        };

        if guess < secret_number {
            println!("📉 너무 작아요!");
        } else if guess > secret_number {
            println!("📈 너무 커요!");
        } else {
            println!("🎉 정답입니다! 숫자는 {}였습니다!", secret_number);
            break;
        }

        if turns >= max_turns {
            println!("❌ 모든 기회를 소진했습니다. 정답은 {}였습니다.", secret_number);
            break;
        }
    }
}
