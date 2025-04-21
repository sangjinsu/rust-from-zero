use std::collections::HashMap;

pub fn run() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    for i in &v {
        println!("v: {}", i);
    }

    let mut scores = HashMap::new();
    scores.insert("영어", 90);
    scores.insert("수학", 80);

    if let Some(score) = scores.get("영어") {
        println!("영어 점수: {}", score);
    }

    let mut scores = HashMap::new();

    scores.insert("수학", 90);
    scores.insert("과학", 85);

    for (subject, score) in &scores {
        println!("{} 점수: {}", subject, score);
    }

    let students = vec!["철수", "영희", "민수"];
    for name in students {
        println!("학생 이름: {}", name);
    }
}
