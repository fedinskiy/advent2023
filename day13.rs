use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::cmp;
use std::fmt;

fn main() -> std::io::Result<()> {

    let row=parse("#.##..##.");
    println!("{:?}", row);
    println!("start");
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut current = Pattern::new();
    lines
    .map(|line|{
        let row:Vec<Symbol>=parse(&line);
        row
        })
    .for_each( |row|
        if row.len()>0{
            current.add(row)
        } else {
            patterns.push(current.clone());
            current=Pattern::new()
        }
    );
    patterns.push(current); //the last one
    let sum=patterns.iter()
    .map(|pattern| {
        pattern.get_mirrors()
    })
    .fold(0, |sum, value| sum+value);
    println!("{:?}", sum);
    // println!("patterns {}: {patterns:?}", patterns.len());
    Ok(())
}

#[derive(Copy, Clone, PartialEq)]
enum Symbol {
    Dot,
    Hash,
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = match self {
            Symbol::Dot => '.',
            Symbol::Hash => '#'
        };
        write!(f, "{}", source)
    }
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Symbol::Hash,
            '.' => Symbol::Dot,
            _ => panic!("Unknown tile: {}!", c)
        }
    }
}
#[derive(Clone)]
struct Pattern {
    conditions: Vec<Vec<Symbol>>,
}

impl Pattern {
    fn add(&mut self, condition:Vec<Symbol>) {
        self.conditions.push(condition)
    }

    fn new() ->Self {
        Pattern {conditions: Vec::new()}
    }

    fn get_mirrors(&self)->usize {  
        println!("Processing {:?}", &self);
        if let Some(horisontal) =self.get_horisontal_mirror() {
            return horisontal;
        }
        if let Some(vertical)=self.get_vertical_mirror() {
            return vertical*100;
        } else {
            panic!("No mirror found!");
        }
    }
    fn get_horisontal_mirror(&self)->Option<usize> {
        let first:&Vec<Symbol> = &self.conditions[0];
        let mut mirrors:Vec<usize>=Vec::new();
        for position in 1..first.len() { //human-readable positions
            if is_mirror(position, first) {
                mirrors.push(position);
            }
        }
        for row in &self.conditions {
            let mut possible_mirrors:Vec<usize>=Vec::new();
            for position in mirrors {
                if is_mirror(position, row) {
                    possible_mirrors.push(position);
                }
            }
            mirrors=possible_mirrors;
        }
        return mirrors.pop();
    }

    fn get_vertical_mirror(&self)->Option<usize> {
        println!("Computing vertical");
        let mut columns = Columns{current: 0, matrix: self.conditions.clone()};

        let first:Vec<Symbol> = columns.next().expect("First columns exists");
        let mut mirrors:Vec<usize>=Vec::new();
        for position in 1..first.len() { //human-readable positions
            if is_mirror(position, &first) {
                mirrors.push(position);
            }
        }
        println!("Mirrors after the first line: {:?}", mirrors);
        for row in columns {
            let mut possible_mirrors:Vec<usize>=Vec::new();
            for position in mirrors {
                if is_mirror(position, &row) {
                    possible_mirrors.push(position);
                }
            }
            println!("The mirrors: {:?}", possible_mirrors);
            mirrors=possible_mirrors;
        }
        return mirrors.pop();
    }
}

struct Columns {
    current: usize,
    matrix: Vec<Vec<Symbol>>,
}

impl Iterator for Columns {
    type Item=Vec<Symbol>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current>=self.matrix[0].len() {
            return None
        } else {
            let mut answer=Vec::new();
            for row in &self.matrix {
                answer.push(row[self.current]);
            }
            self.current+=1;
            return Some(answer);
        }
        
    }
}

fn is_mirror(after: usize, row: &Vec<Symbol>) -> bool {
    //after: 2
    // >\<
    //12\3456789 — logical numeration
    //#.\##..##.
    //01\2345678 — in-memory numeration
    let symbols_to_left=after;
    let symbols_to_right=row.len()-symbols_to_left;
    let steps=cmp::min(symbols_to_left,symbols_to_right);
    let index=after-1;
    println!("{symbols_to_left}, {symbols_to_right}, {steps}");
    for i in 0..steps {
        if row[index-i]!=row[index+i+1]{
            println!("failed at {} and {} in {row:?}",index-i,index+i+1);
            return false;
        }
        println!("succeeded at {} and {} in {row:?}",index-i,index+i+1);
    }
    println!("Anwer is {after} for {row:?}\n");
    return true;
}

fn parse(input: &str) -> Vec<Symbol> {
    input.chars().map(|c| Symbol::from_char(c)).collect()
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n").unwrap();
        for line in &self.conditions {
            write!(f, "{:?}\n", line).unwrap()
        }
        Ok(())
    }
}

fn show(source: &Pattern) -> () {
    for line in &source.conditions {
        println!("{line:?}\n");
    }
}
