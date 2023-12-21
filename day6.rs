use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::env;

type time=u128;
fn main() -> std::io::Result<()> {
    let x:u128=358_105_418_071_080;
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok);
    let time:time = time::from_str(&lines.next().unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .replace(" ","")).unwrap();

    let distance:String = lines.next().unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .replace(" ","");
    println!("{:?}",time);
    println!("{:?}",distance);

    // let result = get_wins(time,&distance);

    let first_win=get_first_win(time,&distance);
    let calculated=time+1-first_win*2;
    println!("first: {}, expected: {}", first_win, calculated);

    // println!("{:?}",result);
    Ok(())
    
}

fn get_wins(time:time, distance:&str) -> time{
    let mut result=0;
    for charge in 0..=time {
        let race=time-charge;
        let travelled=race*charge;
        dbg!(travelled);
        if more(&travelled.to_string(), distance) {
            result+=1;
        }
    }
    return result;
}

fn get_first_win(time:time, distance:&str) -> time{
    for charge in 0..=time {
        let race=time-charge;
        let travelled=race*charge;
        if more(&travelled.to_string(), distance) {
            return charge;
        }
    }
    return 0;
}


fn more(first: &str, second: &str)->bool {
    if first.len()>second.len() {
        return true;
    }
    if second.len()>first.len() {
        return false;
    }
    for (left, right) in first.chars().zip(second.chars()) {
        if left>right{
            return true;
        } else if right>left {
            return false;
        }
    }
    return false;
}