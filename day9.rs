use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    assert_eq!(5, get_previous(&vec![10,13,16,21,30,45]));
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let sequences:Vec<Vec<i32>> = lines
    .map(|line| read_seq(&line))
    .collect();

    let results:i32 = sequences.iter()
        .map(|seq| get_previous(seq))
        .reduce(|first,second| first+second)
        .unwrap();

    println!("{:?}",results);
    Ok(())
}

fn read_seq (input: &str) -> Vec<i32>{
    input.split(' ')
    .map(|number| i32::from_str_radix(number,10).unwrap())
    .collect()
}


fn get_next (origin: &Vec<i32>) -> i32 {
    let mut all_sequences:Vec<Vec<i32>> = Vec::new();
    all_sequences.push(origin.to_vec());
    while !all_zeroes(all_sequences.last().unwrap()) {
        let previous=all_sequences.last().unwrap();
        let mut current=vec![0;previous.len()-1];
        for i in 0..current.len() {
            current[i]=previous[i+1]-previous[i];
        }
        all_sequences.push(current);
    }
    let mut last_increase=0;
    for seq in all_sequences.iter().rev() {
        last_increase+=seq.last().unwrap();
    }
    return last_increase;
}

fn get_previous(origin: &Vec<i32>) -> i32 {
    let mut all_sequences:Vec<Vec<i32>> = Vec::new();
    all_sequences.push(origin.to_vec());
    while !all_zeroes(all_sequences.last().unwrap()) {
        let previous=all_sequences.last().unwrap();
        let mut current=vec![0;previous.len()-1];
        for i in 0..current.len() {
            current[i]=previous[i+1]-previous[i];
        }
        all_sequences.push(current);
    }
    let mut last_leftmost=0;
    for seq in all_sequences.iter().rev() {
        last_leftmost=seq.first().unwrap()-last_leftmost;
    }
    return last_leftmost;
}

fn all_zeroes (seq: &Vec<i32>) -> bool {
    return seq.iter().all(|&number| number==0);
}