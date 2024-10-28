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
    
    fn move_north(&mut self) {
        for 
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