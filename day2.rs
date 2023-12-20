use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::cmp::max;
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result:u32 =0;
    for mut line in reader.lines().filter_map(Result::ok).filter(|line| !line.is_empty()){
        //println!("{}", line);
        let game = Game::parse(&mut line);
//        println!("{}, {}", game.count, game.power());
        result+=game.power();
        }
   
    println!("{}",result);
    Ok(())
}
#[derive(Debug)]
struct Attempt {
    red: u32,
    green: u32,
    blue: u32,
}
#[derive(Debug)]
struct Game {
    count: u32,
    attempts: Vec<Attempt>
}
impl Attempt {
    fn parse(input: &str)->Self {
        let (mut red, mut green, mut blue)=(0,0,0);
        for line in input.split(',') {
            let (count_str,color) = line.trim().split_once(' ').unwrap();
            let count=u32::from_str(&count_str.trim()).unwrap();
            match color { 
            "red" => red=count,
            "green" => green=count,
            "blue" => blue=count,
            _ => panic!("Unknown color: {}", color)
            }
        }
        Self{
            red: red,
            green: green,
            blue: blue
        }
    }
}
impl Game {
    fn parse(input: &mut String)->Self {
        let (game_info,attempts_info)=input.split_once(':').unwrap();
        let digits=game_info.strip_prefix("Game ").unwrap();
        let count=u32::from_str(&digits).unwrap();
        let attempts=attempts_info.split(';').map(|part| Attempt::parse(part)).collect(); 
        Self {
            count: count,
            attempts: attempts
        }
    }
    fn power(&self)->u32{
        let (mut red, mut green, mut blue)=(0,0,0);

        for attempt in &self.attempts {
                red=max(red,attempt.red);
                green=max(green,attempt.green);
                blue=max(blue,attempt.blue);
            }
        return red*green*blue;
     }
    

    fn is_valid(&self)->bool{
        *&self.attempts.iter().filter(|attempt| {
            attempt.red>12 || attempt.green>13|| attempt.blue>14
        })
        .count()==0
    }
}
