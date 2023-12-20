use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result:u32 =0;
    while let Ok(1..) = reader.read_line(&mut line) {
//        println!("{}", line);
        let first_digit = get_first_number(&line);
        let last_digit = get_last_number(&line);
        result+=first_digit*10+last_digit;
        line.clear();
    }
    println!("{}", result);
    Ok(())
}

fn get_first_number(str: &String) -> u32 {
    return *str.chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .flatten()
        .get_or_insert(0);
}

fn get_last_number(str: &String) -> u32 {
    return *str.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .flatten()
        .get_or_insert(0);
}

