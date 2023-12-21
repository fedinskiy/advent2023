use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;
use std::env;
fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut cards:Vec<usize>=Vec::new();
    let mut total_cards:usize=0;
    for (number,line) in reader.lines().filter_map(Result::ok).filter(|line| !line.is_empty()).enumerate(){
        if number+1>cards.len() {
            cards.resize(number+1,0);
        }
        cards[number]+=1;
        let repeats=cards[number];
        let card = Card::parse(&line);
        let wins:usize=card.wins();
        let next=number+1;
        if next+wins>cards.len() {
            cards.resize(next+wins,0);
        }
        // println!("card:{}, wins: {}, state:{:?}",number, wins, cards);
        for i in next..next+wins{
            cards[i]+=repeats;
        }
        // println!("{:?}",cards);
        total_cards=number;
    }
   
    println!("{:?}, {}",cards, total_cards);
    let mut result:usize=0;
    for i in 0..=total_cards{
        result+=cards[i];
    }
    println!("{}",result);
    Ok(())
}
#[derive(Debug)]
struct Card {
    order: u32,
    my: HashSet<u32>,
    winning: HashSet<u32>,
}
impl Card {
    fn parse(input: &String)->Self {
        let (game_info,numbers)=input.split_once(':').unwrap();
        let digits=game_info.strip_prefix("Card ").unwrap().trim();
        let order=u32::from_str(&digits).unwrap();
        let (my_numbers, winning_numbers)=numbers.split_once('|').unwrap();
        let my:Vec<_>=my_numbers.split(' ')
        .filter(|value| !value.is_empty())
        .map(|number| u32::from_str(number).unwrap())
        .collect();
        let winning:Vec<_>=winning_numbers
        .split(' ')
        .filter(|value| !value.is_empty())
        .map(|number| u32::from_str(number).unwrap())
        .collect(); 
        Self {
            order,
            my: HashSet::<u32>::from_iter(my),
            winning: HashSet::<u32>::from_iter(winning)
        }
    }
    fn wins(&self)->usize{
        return self.my.intersection(&self.winning).count();
    }

    fn worth(&self)->u32{
        let win_count:u32=self.wins().try_into().unwrap();
        if win_count==0 {
            return 0 ;
        }
        return 2_u32.pow(win_count-1);
    }
}
