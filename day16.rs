use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;
use std::str::FromStr;


fn main() -> std::io::Result<()> {
    println!("start");
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines:Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();
    let layout=Layout{objects: lines};
    println!("Max: {}", get_max_result(&layout));
    Ok(())
}

fn get_max_result(field: &Layout) -> usize {
    let mut max=0;
    let vertical=field.objects.len();
    let horisontal=field.objects[0].len();

    let mut starts: Vec<(Direction,Position)> = Vec::with_capacity(vertical*2+horisontal*2);
    
    //left side
    for i in 0..vertical {
        starts.push((Direction::FromLeft, Position(i,0)));
    }
    //right side
    for i in 0..vertical {
        starts.push((Direction::FromRight, Position(i,horisontal-1)));
    }
    //top
    for i in 0..horisontal {
        starts.push((Direction::Top, Position(0,i)));
    }
    //bottom
    for i in 0..horisontal {
        starts.push((Direction::Below, Position(vertical-1,i)));
    }
    println!("{field:?}");
    println!("{starts:?}");
    for start in starts {
        let (direction,position)=start;
        let energised=Energized::create_for(&field);
        let energised=process(&field, energised, direction, position);
        let count= energised.count();
        // println!("{energised:?}");
        println!("{}", count);
        if count>max {
            max=count;
        }
    }

    max
}

#[derive(Clone)]
struct Layout {
    objects: Vec<String>
}

#[derive(Clone)]
// #[derive(Debug)]
struct Energized {
    field: Vec<Vec<bool>>
}

impl Energized {
    fn count(&self) -> usize {
        self.field.iter()
            .flat_map(|line| line.iter())
            .filter(|value| **value==true)
            .count()
    }
}

impl fmt::Debug for Energized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n").unwrap();
        for line in &self.field {
            for energised in line {
                if *energised {
                    f.write_str("#").unwrap(); 
                } else {
                    f.write_str(".").unwrap(); 
                }
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}

impl Energized {
    fn create_for(field: &Layout) -> Self  {
        let vertical=field.objects.len();
        let horisontal=field.objects[0].len();
        let content=vec![vec![false; horisontal];vertical];
        Energized{field: content}
    }
}


impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n").unwrap();
        for line in &self.objects {
            write!(f, "{}\n", line).unwrap()
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Top,
    Below,
    FromLeft,
    FromRight
}

impl Direction {
    fn mirror_rising(&self) -> Direction {
        // "/"
        return match self {
            Direction::Top => Direction::FromRight,
            Direction::Below => Direction::FromLeft,
            Direction::FromLeft => Direction::Below,
            Direction::FromRight => Direction::Top,
        }
    }

    fn mirror_falling(&self) -> Direction {
        // "\"
        return match self {
            Direction::Top => Direction::FromLeft,
            Direction::Below => Direction::FromRight,
            Direction::FromLeft => Direction::Top,
            Direction::FromRight => Direction::Below,
        }
    }
}

struct Position(usize, usize);

impl Position {
    fn move_to(&self, from: Direction) ->Position  {
        let &Position(vertical, horisontal)=self;
        return match from {
            Direction::Top => Position(vertical+1, horisontal),
            Direction::Below => Position(vertical-1, horisontal),
            Direction::FromLeft => Position(vertical, horisontal+1),
            Direction::FromRight => Position(vertical, horisontal-1),
        }
    }

    fn will_hit_wall(&self, from: Direction) ->bool  {
        let &Position(vertical, horisontal)=self;
        return (from==Direction::Below && vertical==0) || (from==Direction::FromRight && horisontal==0)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vertical = self.0;
        let horisontal = self.1;
        write!(f, "({vertical},{horisontal})")
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vertical = self.0+1;
        let horisontal = self.1+1;
        write!(f, "({vertical},{horisontal})")
    }
}



fn process(target: &Layout, mut results: Energized, beam: Direction, current: Position) -> Energized {
    let Position(vertical, horisontal)=current;
    let fields=&target.objects;
    if vertical>=fields.len() || horisontal>=fields[vertical].len() {
       return results
    }
    let is_active=results.field[vertical][horisontal];
    results.field[vertical][horisontal]=true;
    let field_type=fields[vertical].chars().nth(horisontal).unwrap();
    // println!("{target:?}");
    // println!("{results:?}");
    // todo early return for activated splitters
    match (is_active, field_type){
        (true, '|' | '-') => return results,
        _=> ()
    }
    match field_type {
        '.' => {
            if current.will_hit_wall(beam) {
                return results;
            }
            let next: Position = current.move_to(beam);
            return process(target, results, beam, next);
        },
        '/' => {
            let mirrored:Direction = beam.mirror_rising();
            if current.will_hit_wall(mirrored) {
                return results;
            }
            let next: Position = current.move_to(mirrored);
            return process(target, results, mirrored, next);
        },
        '\\' => {
            let mirrored:Direction = beam.mirror_falling();
            if current.will_hit_wall(mirrored) {
                return results;
            }
            let next: Position = current.move_to(mirrored);
            return process(target, results, mirrored, next);
        },
        '|' => {
            match beam {
                Direction::Top | Direction::Below => { // beam passes through pointy ends
                    if current.will_hit_wall(beam) {
                        return results;
                    }
                    let next: Position = current.move_to(beam);
                    return process(target, results, beam, next);
                },
                _ => { //beam splits
                    let split_Top=Direction::Top;
                    if current.will_hit_wall(split_Top) {
                        return results;
                    }
                    let half_results=process(target, results, split_Top, current.move_to(split_Top));
                    let split_Below=Direction::Below;
                    if current.will_hit_wall(split_Below) {
                        return half_results;
                    }
                    return process(target, half_results, split_Below, current.move_to(split_Below))
                }
            }
        },
        '-' => {
            match beam {
                Direction::FromLeft | Direction::FromRight => { // beam passes through pointy ends
                    if current.will_hit_wall(beam) {
                        return results;
                    }
                    let next: Position = current.move_to(beam);
                    return process(target, results, beam, next);
                },
                _ => { //beam splits
                    let split_FromLeft=Direction::FromLeft;
                    if current.will_hit_wall(split_FromLeft) {
                        return results;
                    }
                    let half_results=process(target, results, split_FromLeft, current.move_to(split_FromLeft));
                    let split_FromRight=Direction::FromRight;
                    if current.will_hit_wall(split_FromRight) {
                        return half_results;
                    }
                    return process(target, half_results, split_FromRight, current.move_to(split_FromRight))
                }
            }
        },
        _ => panic!("Unknown symbol: {}", field_type)
    }
}
