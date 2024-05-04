use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    tests();
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
    let (left, right)=collect_surroundings(&longest_path, &labirint);
    // println!("{:?}", left);
    to_inner_tile(left, &labirint,&longest_path).iter().for_each(|tile| {
        // show_path_and_tile(&labirint,&longest_path, &tile);
        println!("{}",tile.len());
    });

    to_inner_tile(right, &labirint,&longest_path).iter().for_each(|tile| {
        // show_path_and_tile(&labirint,&longest_path, &tile);
        println!("{}",tile.len());
    });
   
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

fn to_inner_tile(fields: Vec<Coordinates>, labirint:&Vec<Vec<Cell>>, path: &Path) -> Option<HashSet<Coordinates>> {
    let max_y=labirint.len();
    let max_x=labirint[0].len();
    let fields_on_path:HashSet<Coordinates>=path.0.iter()
                                            .map(|movement| Coordinates{..movement.to})
                                            .collect();
    let mut tile:HashSet<Coordinates>=HashSet::new();
    for field in fields {
        if fields_on_path.contains(&field) {
            continue; //these fields are here by mistake
        }
        if field.x==0 || field.y==0 || field.x==max_x ||  field.y==max_y {
            return None; //we found the outer tile
        }
        tile.insert(field);
    }
    let mut insert_happened: bool=true;
    while insert_happened {
        let mut new_fields:HashSet<Coordinates>=HashSet::new();
        for field in &tile {
            for direction in [Connection::North, Connection::West, Connection::East, Connection::South] {
                let neighbor = field.get_neighbor(&direction);
                if neighbor.x==0 || neighbor.y==0 || neighbor.x==max_x ||  neighbor.y==max_y {
                    return None; //we found the outer tile, no need to process it further.
                }
                if !fields_on_path.contains(&neighbor) {
                    new_fields.insert(neighbor);
                }
            }
        }
        // show_path_and_tile(&labirint,&path, &tile);
        // println!("fields: {:?}",new_fields );
        insert_happened=false;
        for new_field in new_fields {
            insert_happened|=tile.insert(new_field);
        }
    }

    return Some(tile)
}
fn collect_surroundings(path: &Path, labirint:&Vec<Vec<Cell>>) -> (Vec<Coordinates>, Vec<Coordinates>) {
        let mut left=Vec::new();
        let mut right=Vec::new();
        for current_move in &path.0 {
            let cell:&Cell=labirint
                .get(current_move.to.y)
                .map(|line| line.get(current_move.to.x))
                .flatten()
                .unwrap(); //all cells from the path should be in the labirint
            let (to_left, to_right) = cell.get_neighbors(current_move.direction);
            for left_connection in to_left {
                left.push(current_move.to.get_neighbor(&left_connection));
            }
            for right_connection in to_right {
                right.push(current_move.to.get_neighbor(&right_connection));
            }
        }
        (left, right)

    //todo calucalate which form has the S, and get surroundings for it
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
        .get(y)
        .map(|line| line.get(x))
        .flatten() // we may have reached the edge
        .map(|cell| {
            cell.get_direction(cause.direction)})
        .flatten() else { // we may have reached the ground or the start
            return None;
        };
        let next: Coordinates=cause.to.get_neighbor(&direction);

    Some(Move{ to: next, direction: direction})
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize
}

impl Coordinates {
    fn get_neighbor(&self, side: &Connection) -> Coordinates {
        let (x,y) = (self.x, self.y);
        let (new_x, new_y) = match side {
            Connection::West => (x-1, y),
            Connection::North => (x, y-1),
            Connection::East => (x+1, y),
            Connection::South => (x, y+1)
        };
        Coordinates{x: new_x, y: new_y}
    }
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.y+1, self.x+1)
    }
}
impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Connection {
    West,
    North,
    East,
    South
}
impl Connection {
    fn connects(&self, other: &Self)-> bool {
        self.opposite()==*other
    }

    fn clockwise(&self)-> Self {
        match self {
            Connection::West => Connection::North,
            Connection::East => Connection::South,
            Connection::North => Connection::East,
            Connection::South => Connection::West,
        }
    }
    fn counterclockwise(&self)-> Self {
        match self {
            Connection::West => Connection::South,
            Connection::East => Connection::North,
            Connection::North => Connection::West,
            Connection::South => Connection::East,
        }
    }
    fn opposite(&self)-> Self {
        match self {
            Connection::West => Connection::East,
            Connection::East => Connection::West,
            Connection::North => Connection::South,
            Connection::South => Connection::North,
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
        if first.connects(&entrance) {
            return Some(*second)
        } else if second.connects(&entrance) {
            return Some(*first)
        } else {
            return None
        }
    }

    fn get_neighbors(&self, external: Connection) -> (Vec<Connection>, Vec<Connection>) {
        let mut left:Vec<Connection>=Vec::new();
        let mut right:Vec<Connection>=Vec::new();
        let (first, second) = match self { 
            Cell::Pipe(first, second) => (first, second),
            _ => return (left, right) // none-pipes are here by mistake, so return empty
        };
        let is_straight = first.connects(second); // | or  -
        let entrance = external.opposite(); // this method should process only connected Cells
        let exit = if &entrance == first {second} else {first};
        // println!("Straight {}, entrance: {:?}, exit: {:?}", is_straight, entrance, exit);
        if is_straight {
            left.push(entrance.clockwise());
            right.push(entrance.counterclockwise());
        } else {
            if &entrance.clockwise() == exit { // eg it is 7, when coming from below/South
                right.push(entrance.counterclockwise());
                right.push(entrance.opposite());
            } else { // eg it is F, when coming from below/South
                left.push(entrance.clockwise());
                left.push(entrance.opposite());
            }
        }
        (left, right)
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

fn show_path_and_tile(labirint: &Vec<Vec<Cell>>, path: &Path, tile: &HashSet<Coordinates>) {
    let fields_on_path:HashSet<Coordinates>=path.0.iter()
        .map(|movement| Coordinates{..movement.to})
        .collect();
    for (y, row) in labirint.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            let current_coordinates=Coordinates{y, x};
            if fields_on_path.contains(&current_coordinates) {
                print!("{}", field);
            } else if tile.contains(&current_coordinates) {
                print!("I");
            } else {
                print!(".")
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

fn tests() {
	let cell = Cell::from_char('L');
	assert_eq!(cell.get_direction(Connection::South), Some(Connection::East));
	assert_eq!(cell.get_direction(Connection::West), Some(Connection::North));
    let (left, right) = cell.get_neighbors(Connection::South);
    assert!(left.is_empty());
    assert_eq!(right, vec!(Connection::West, Connection::South));

    let (left, right) = cell.get_neighbors(Connection::West);
    assert!(right.is_empty());
    assert_eq!(left, vec!(Connection::South, Connection::West));
}