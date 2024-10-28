use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;

fn main() -> std::io::Result<()> {

    let row=parse("#.##.O.##.");
    println!("{:?}", row);
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
    println!("{:?}", platform.transposed().move_left().transposed().load());
    Ok(())
}

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