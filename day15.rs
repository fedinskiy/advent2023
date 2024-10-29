use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;

fn main() -> std::io::Result<()> {
    assert_eq!(hash("HASH"), 52);
    println!("start");
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines:Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();
    let steps:Vec<String>=lines.iter()
        .flat_map(|line| line.split(',').map(str::to_owned))
        .collect();
    println!("{steps:?}");
    let sum=steps.iter()
        .map(|line| hash(&line))
        .fold(0 as u64, |sum, hash| sum+hash as u64 );
    println!("{sum:?}");
    // debug(&platform,2);
    // debug(&platform,9);
    Ok(())
}

fn hash(input:&str) -> u16  {
    let mut current:u16=0;
    for byte in input.bytes() {
        current+=byte as u16;
        current*=17;
        current%=256;
    }
 
    return current
}