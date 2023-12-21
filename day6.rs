use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::env;
fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok);
    let times:Vec<u32> = lines.next().unwrap()
    .split(" ")
    .skip(1)
    .filter(|line| !line.is_empty())
    .map(|number| u32::from_str(&number).unwrap())
    .collect();

    let distances:Vec<u32> = lines.next().unwrap()
    .split(" ")
    .skip(1)
    .filter(|line| !line.is_empty())
    .map(|number| u32::from_str(&number).unwrap())
    .collect();
   
   let result:u32=times.iter().zip(&distances)
   .map(|(&time,&distance)| {
        return get_wins(time,distance);
   })
   .reduce(|el1, el2| el1*el2).unwrap();

    println!("{:?}",times);
    println!("{:?}",distances);
    println!("{:?}",result);
    Ok(())
}

fn get_wins(time:u32, distance:u32) -> u32{
    let mut result=0;
    for charge in 0..=time {
        let race=time-charge;
        let travelled=race*charge;
        if travelled>distance {
            result+=1;
        }
    }
    return result;
}
