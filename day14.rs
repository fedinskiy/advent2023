use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;

fn main() -> std::io::Result<()> {
    println!("start");
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let mut platform = Platform::new();
    lines
    .map(|line|{
        let row:Vec<Stone>=parse(&line);
        row
        })
    .for_each( |row|
        platform.add(row)
    );
    println!("{platform:?}");
    // debug(&platform,2);
    // debug(&platform,9);
    println!("{:?}", get_answer(&platform));
    Ok(())
}

fn debug(platform: &Platform, cycles: usize) {
    let mut debugged=platform.clone();
    for _ in 0..=cycles {
        debugged=debugged.cycled();   
    }
    println!("After {cycles}:{:?} \n {debugged:?}", debugged.load());
}

fn get_answer(source: &Platform) -> usize {
    let mut hashes: Vec<usize>=Vec::new();
    let mut loads: Vec<usize>=Vec::new();
    let mut platform=source.clone();
    let repeats=1000000000;
    // let repeats=20;
    for current in 0..repeats {
        if current%100==0 {
            println!("Processing {current}");
        }
        platform=platform.cycled();
        loads.push(platform.load());
        let current_hash=platform.load()+2*platform.transposed().load();
        match hashes.iter().position(|&hash| hash==current_hash) {
            None => hashes.push(current_hash),
            Some(shift) => {
                println!("Found repeat ({}) on positions {} and {}",
                current_hash, shift, current);
                println!("{hashes:?}");
                println!("{loads:?}");
                
                let cycled_values=repeats-shift;
                println!("{cycled_values}");
                let cycle_length=current-shift;
                println!("{cycle_length}");
                let phase=cycled_values%cycle_length;
                println!("{phase}");
                let answer=loads[phase+shift-1];
                println!("Early return: {}", answer);
                return answer;
            }
        }
    }
    platform.load()
}
//  0   1   2   3   4   5    6  7   8   9   10  11  12
//  271 235 223 211 199 202 209 217 228 223 211 199 202, 209, 217, 228, 223, 211, 199]
//  87  69  69  69  65  64  65  63  68  69  69  65  64, 65, 63, 68, 69, 69, 65, 64]


#[derive(Copy, Clone, PartialEq)]
enum Stone {
    Empty,
    Round,
    Square
}

impl fmt::Debug for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self {
            Stone::Empty => '.',
            Stone::Square => '#',
            Stone::Round => 'O',
        };
        write!(f, "{}", source)
    }
}

impl Stone {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Stone::Square,
            '.' => Stone::Empty,
            'O' => Stone::Round,
            _ => panic!("Unknown tile: {}!", c)
        }
    }
}
#[derive(Clone)]
struct Platform {
    spaces: Vec<Vec<Stone>>,
}

impl Platform {
    fn add(&mut self, condition:Vec<Stone>) {
        self.spaces.push(condition)
    }

    fn new() ->Self {
        Platform {spaces: Vec::new()}
    }

    fn cycled(&self) -> Self {
        self
        .transposed().move_left() //move north
        .transposed().move_left() //move west
        .inverted().transposed().move_left() //move south
        .inverted().transposed().move_left() //move east
        .inverted().mirrored() //north to the top
    }

    fn transposed(&self) -> Self {
        let mut transposed=Platform::new();
        for column in 0..self.spaces[0].len() {
            let mut new_row=Vec::new();
            for old_row in &self.spaces {
                new_row.push(old_row[column]);
            }
            transposed.spaces.push(new_row);
        }
        return transposed;
    }

    fn inverted(&self) -> Self {
        let mut inverted=Platform::new();
        for row in self.spaces.iter().rev() {
            inverted.spaces.push(row.clone())
        }
        return inverted;
    }

    fn mirrored(&self) -> Self {
        let mut result=Platform::new();
        for row in self.spaces.iter() {
            result.spaces.push(row.clone()
            .into_iter()
            .rev()
            .collect())
        }
        return result;
    }
    
    fn move_left(&self) -> Self {
        let mut moved=Platform::new();

        for row in &self.spaces {
            let mut arranged:Vec<Stone>=Vec::new();
            let mut first_empty:usize=0;
            for (position, symbol) in row.iter().enumerate() {
                // println!("{symbol:?}, [{position:?}], {first_empty:?}");
                match symbol {
                    Stone::Empty => arranged.push(Stone::Empty),
                    Stone::Square => {
                        arranged.push(Stone::Square);
                        first_empty=position+1;
                    },
                    Stone::Round =>{
                        arranged.push(Stone::Empty);
                        arranged[first_empty]=Stone::Round;
                        first_empty=first_empty+1;
                    },
                }
                // println!("{symbol:?}, [{position:?}], {first_empty:?}\n");
            }
            moved.spaces.push(arranged);
        }

        moved
    }

    fn load(&self) -> usize {
        let total_height=self.spaces.len();
        let mut sum:usize=0;

        for (row, content) in self.spaces.iter().enumerate(){
            let multiplier=total_height-row;
            for position in content {
                if *position==Stone::Round {
                    sum+=multiplier;
                }
            }
        }
        sum
    }
} 

fn parse(input: &str) -> Vec<Stone> {
    input.chars().map(|c| Stone::from_char(c)).collect()
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n").unwrap();
        for line in &self.spaces {
            write!(f, "{:?}\n", line).unwrap()
        }
        Ok(())
    }
}