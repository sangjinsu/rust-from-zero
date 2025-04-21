use std::fs::File;
use std::io::{self, Read};

pub fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // 실패 시 바로 리턴
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
