use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::env;
use std::cmp::Ord;
use std::cmp::Ordering;
use crate::Card::J;
use crate::Card::_8;

fn main() -> std::io::Result<()> {

    // 248916757 — too low 
    // 249796054 — too high
    // 249641493
    assert_eq!(Hand::calculate_type([J,J,_8,J,J]), HandType::Five);
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok);
    let mut set:Vec<Hand>=lines
    .map(|line| Hand::parse(&line))
    .collect();

    // println!("{:?}",set);
    set.sort_unstable_by(|this, other| {
        if this._type>other._type {
            return Ordering::Greater;
        } else if  this._type<other._type {
            return Ordering::Less;
        }
        for i in 0..5 {
            if this.cards[i]!=other.cards[i] {
                if this.cards[i]>other.cards[i]{
                    return Ordering::Greater;
                } else if this.cards[i]<other.cards[i] {
                    return Ordering::Less; 
                }
            }
        }
        return Ordering::Equal;
    });
    let mut result:u32 =0;
    for (order,hand) in set.iter().enumerate() {
         println!("{:?} {:?}", hand.cards, hand._type);
        result+=((order as u32)+1)*hand.bid;
    }
    println!("{:?}",result);
    Ok(())
    
}
#[derive(Debug)]
struct Hand{
    cards: [Card;5],
    _type: HandType,
    bid: u32
}
impl Hand {
    fn parse(source: &str) -> Self {
        let (cards_string, bid) = source.split_once(' ').unwrap();
        let cards:[Card;5]=cards_string.chars()
        .map(|card| Card::parse(card))
        .collect::<Vec<Card>>()
        .try_into().unwrap();
        Self {
            _type: Self::calculate_type(cards),
            cards: cards,
            bid: u32::from_str(&bid).unwrap()}
    }

    fn calculate_type(cards: [Card;5]) -> HandType{
        let mut counter:Vec<u8>=[0].repeat(Card::highest_card()+1);
        for card in cards{
            counter[card.strength()]+=1;
        }
        let mut has_triple=false;
        let mut pair_count=0;

        let joker_count=counter[Card::J.strength()];
        counter[Card::J.strength()]=0;
        if joker_count==5 {
            return HandType::Five;
        }
        for count in counter {
            if count!=0 {
                match count+joker_count {
                    5 => return HandType::Five,
                    4 => return HandType::Four,
                    _ => ()
                }
            }   
            match count {
                3 => has_triple=true,
                2 => pair_count+=1,
                _ => ()
            }
        } 

        assert!(!(has_triple&&joker_count>0));
        assert!(joker_count<3, "Failed to parse {:?}", cards);
        if has_triple && pair_count>0 {
            return HandType::FullHouse;
        }
        if has_triple && pair_count==0 {
            return HandType::Three;
        }
        if pair_count>=2 {
            if joker_count > 0 {
                return HandType::FullHouse;
            } else {
                return HandType::TwoPairs;
            }
        }
        if pair_count==1 {
            match joker_count {
                2 => return HandType::FullHouse,
                1 => return HandType::Three,
                0 => return HandType::Pair,
                _ => panic!("We can not calculate cards: {:?}", cards)
            }
        }
        match joker_count {
            2 => return HandType::Three,
            1 => return HandType::Pair,
            0 => return HandType::High,
            _ => panic!("We can not calculate cards: {:?}", cards)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self._type>other._type {
            return Ordering::Greater;
        } else if  self._type<other._type {
            return Ordering::Less;
        }
        for i in 0..5 {
            if self.cards[i]!=other.cards[i] {
                if self.cards[i]>other.cards[i]{
                    return Ordering::Greater;
                } else if self.cards[i]<other.cards[i] {
                    return Ordering::Less; 
                }
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other)==Ordering::Equal;
    }
}

impl Eq for Hand {
}
#[derive(Debug, Copy, Clone,PartialEq,PartialOrd)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5= 5,
    _4= 4,
    _3 = 3,
    _2 = 2
}
impl Card {
    fn parse(source: char) -> Self {
        match source {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => panic!("Unknown card: {}", source)
        }
    }
    fn strength(self)->usize {
        return self as usize;
    }
    fn highest_card()->usize {
        Card::A as usize
    }
}
#[derive(Debug,PartialEq,PartialOrd)]
enum HandType {
    Five=7,
    Four=6,
    FullHouse=5,
    Three=4,
    TwoPairs=3,
    Pair=2,
    High=1
}
