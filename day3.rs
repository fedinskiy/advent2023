use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
//use std::collections::HashSet;
use std::env;
use std::cmp;

fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut result=0;
    let mut previous=Window{first: None, central: None, last: None};

    for (number, line) in reader.lines().filter_map(Result::ok).enumerate(){
        let current=Window{
            first:previous.central,
            central:previous.last,
            last: Some(LineInfo::init(number, line))
        };
        // if let Some(ref line)=current.central {
        //     println!("{}: {:?} {:?}", number, line.content, &line.symbols);
        // }
        result+=current.count_gears_ratio();
        previous=current;
    }

    println!("{:?}",result);
    Ok(())
}

fn main_new() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines:Vec<Vec<char>>=Vec::new();
    let mut parsed:Vec<LineInfo>=Vec::new();

    for (number, line) in reader.lines().filter_map(Result::ok).enumerate(){
        lines.push(line.chars().collect());
        parsed.push(LineInfo::init(number, line));
    }
    
    for (number, line) in parsed.iter().enumerate() {
        println!("{}, {:?}",number, line.content);
        for part in &line.parts {
            let left=if part.start>1 {part.start-1} else {0};
            let right=cmp::min(part.end+1, line.content.len());
            println!("{}: {}/{}", part.content, left, right);
        }
    }
    Ok(())
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Part{
    start: usize,
    end: usize,
    content: u32,
    line_id: usize
}
#[derive(Debug)]
struct Symbol {
    column: usize,
    is_gear: bool
}
#[derive(Debug)]
struct LineInfo {
    number: usize,
    content: Vec<char>,
    parts: Vec<Part>,
    symbols: Vec<Symbol>
}
impl LineInfo {
    fn init(number:usize, content:String)->Self{
        let mut parts=Vec::new();
        let mut start: usize=0;
        let mut current_number=String::new();
        let mut symbols = Vec::new();
        for (column,symbol) in content.chars().enumerate() {
            if symbol.is_digit(10){
                if current_number.is_empty(){
                    start=column;
                }
                current_number.push(symbol);
            } else if !current_number.is_empty(){
                    parts.push(Part{start: start,
                        end: column-1, //the number ended on the previous position
                        content: u32::from_str(&current_number).unwrap(),
                        line_id: number
                    });
                    current_number.clear();
            }
            if !symbol.is_digit(10) && symbol!='.' {
                symbols.push(Symbol{column, is_gear:symbol=='*'});
            }
       }
       if !current_number.is_empty(){
        parts.push(Part{start: start,
            end: content.len(), // the number ends on the end of the line
            content: u32::from_str(&current_number).unwrap(),
            line_id: number
        });
        current_number.clear();
}
        Self{number: number, content:content.chars().collect(), parts:parts, symbols: symbols}
    }
}
type Line=Option<LineInfo>;
#[derive(Debug)]
struct Window {
    first: Line,
    central: Line,
    last: Line
}

impl Window {
    fn process(&self)->Vec<&Part>{
        if let Some(central)=&self.central {
        return central.symbols.iter()
            .flat_map(|symbol|{
                let mut results=Vec::new();
                let (left, right)=(symbol.column-1,symbol.column+1);
                    central.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result));
                if let Some(first)=&self.first{
                    first.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result))
                }
                if let Some(last)=&self.last{
                    last.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result));
                }
                return results;
            })
            .collect()
        }
        return Vec::new();
    }

    fn count_gears_ratio(&self)->u32{
        if let Some(central)=&self.central {
        return central.symbols.iter()
            .map(|symbol: &Symbol|{
                let mut results=Vec::new();
                if !symbol.is_gear{
                    return 0;
                }
                let (left, right)=(symbol.column-1,symbol.column+1);
                    central.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result));
                if let Some(first)=&self.first{
                    first.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result))
                }
                if let Some(last)=&self.last{
                    last.parts.iter()
                        .filter(|part| !(part.end<left||right<part.start))
                        .for_each(|result| results.push(result));
                }
                if results.len()!=2{
                    return 0; //this is not a gear
                }
                let ratio:u32=results.get(0).unwrap().content*results.get(1).unwrap().content;
                return ratio;
            })
            .fold(0, |sum, ratio| sum+ratio)
        }
        return 0;
    }
}

                     
