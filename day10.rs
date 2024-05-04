use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;

fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let labirint:Vec<Vec<Cell>> = lines
    .map(|line| parse(&line))
    .collect();
    let Some(start) = find_start(&labirint) else {
        panic!("Start position was not found!")
    };
    println!("Start at {:?}", start);
    let first_moves=vec!(
        Move{to: Coordinates{x: start.x, y: start.y+1}, direction: Connection::South},
        Move{to: Coordinates{x: start.x, y: start.y-1}, direction: Connection::North},
        Move{to: Coordinates{x: start.x-1, y: start.y}, direction: Connection::West},
        Move{to: Coordinates{x: start.x+1, y: start.y}, direction: Connection::East}
    );

    let mut longest_path=Path::empty();
    for first_move in first_moves {
        let calculated_path=get_path(&labirint, first_move);
        if calculated_path.len()>longest_path.len(){
            longest_path=calculated_path;
        }
    }
    // println!("{}",longest_path);
    println!("{}",longest_path.len()/2);

    Ok(())
}

struct Path(Vec<Move>);

impl Path {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn empty()->Self {
        Path(Vec::new())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ",\n")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn get_path(labirint:&Vec<Vec<Cell>>, first_move: Move) -> Path {
    let mut current_move=Some(first_move);
    let mut path:Vec<Move>=Vec::new();
    while let Some(real_move)=current_move {
        // println!("{}", real_move);
        
        current_move=get_next_move(labirint, &real_move);
        path.push(real_move);
    }
    return Path(path);
}
fn get_next_move(labirint:&Vec<Vec<Cell>>, cause: &Move) -> Option<Move> {
    let (x,y) = (cause.to.x, cause.to.y);
    let Some(direction) = labirint
        .get(cause.to.y)
        .map(|line| line.get(cause.to.x))
        .flatten() // we may have reached the edge
        .map(|cell| {
            cell.get_direction(cause.direction)})
        .flatten() else { // we may have reached the ground or the start
            return None;
        };
    let (next_x, next_y) = match direction {
        Connection::West => (x-1, y),
        Connection::North => (x, y-1),
        Connection::East => (x+1, y),
        Connection::South => (x, y+1)
    };
    Some(Move{ to: Coordinates{x: next_x, y: next_y}, direction: direction})
}

struct Coordinates {
    x: usize,
    y: usize
}
impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.y, self.x)
    }
}

struct Move {
    to: Coordinates,
    direction: Connection
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, after going {:?}", self.to, self.direction)
    }
}

fn parse(input: &str) -> Vec<Cell> {
        input.chars().map(|c| Cell::from_char(c)).collect()
}

fn find_start(labirint: &Vec<Vec<Cell>>) -> Option<Coordinates> {
    for (row_number, row) in labirint.iter().enumerate() {
        for (column_number, cell) in row.iter().enumerate() {
            match cell {
                Cell::Start => return Some(Coordinates{x: column_number, y: row_number}),
                _ => ()
            }
        }
    }
    return None
}

#[derive(Debug, Copy, Clone)]
enum Connection {
    West,
    North,
    East,
    South
}
impl Connection {
    fn connects(self, other: Self)-> bool {
        match (self, other) {
            (Connection::West, Connection::East) => true,
            (Connection::East, Connection::West) => true,
            (Connection::North, Connection::South) => true,
            (Connection::South, Connection::North) => true,
            _ => false
        }
    }
}
#[derive(Debug)]
enum Cell {
    Start,
    Ground,
    Pipe(Connection, Connection)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self {
            Cell::Pipe(Connection::North, Connection::South) => '|',
            Cell::Pipe(Connection::West, Connection::East) => '-',
            Cell::Pipe(Connection::North, Connection::East) => 'L',
            Cell::Pipe(Connection::West, Connection::North) => 'J',
            Cell::Pipe(Connection::West, Connection::South) => '7',
            Cell::Pipe(Connection::East, Connection::South) => 'F',
            Cell::Start => 'S',
            Cell::Ground => '.',
            _ => '?'
        };
        write!(f, "{}", source)
    }
}

impl Cell {
    fn get_direction(&self, entrance: Connection)-> Option<Connection> {
        // println!("{:?}, going {:?}", self, entrance);
        let (first, second) = match self {
            Cell::Start => return None,
            Cell::Ground => return None,
            Cell::Pipe(first, second) => (first, second)
        };
        if first.connects(entrance) {
            return Some(*second)
        } else if second.connects(entrance) {
            return Some(*first)
        } else {
            return None
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            '|' => Cell::Pipe(Connection::North, Connection::South),
            '-' => Cell::Pipe(Connection::West, Connection::East),
            'L' => Cell::Pipe(Connection::North, Connection::East),
            'J' => Cell::Pipe(Connection::West, Connection::North),
            '7' => Cell::Pipe(Connection::West, Connection::South),
            'F' => Cell::Pipe(Connection::East, Connection::South),
            'S' => Cell::Start,
            '.' => Cell::Ground,
            _ => panic!("Unknown tile: {}!", c)
        }
    }
}
