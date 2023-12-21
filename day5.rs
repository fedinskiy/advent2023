use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;

use std::env;
use std::fmt;
use std::cmp;
fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);


    let mut lines=reader.lines()
            .filter_map(Result::ok);
    let current:String=lines.next().unwrap();

    let seeds:Vec<u64>=current.strip_prefix("seeds: ").unwrap()
    .split(' ')
    .map(|number| u64::from_str(number).unwrap())
    .collect();

    let mut seed_ranges:Vec<SeedRange>=Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        println!("{}", seeds[i]);
        seed_ranges.push(SeedRange{start: seeds[i], step: seeds[i+1]})
    }
    
    // dbg!(&seed_ranges);
    
    let mut lines=lines.skip(1);
    let seed_to_soil:Map=Map::init(&mut lines);
    let soil_to_fertilizer:Map=Map::init(&mut lines);
    let fertilizer_to_water:Map=Map::init(&mut lines);
    let water_to_light:Map=Map::init(&mut lines);
    let light_to_temperature:Map=Map::init(&mut lines);
    let temperature_to_humidity:Map=Map::init(&mut lines);
    let humidity_to_location:Map=Map::init(&mut lines);
    
    // dbg!(&soil_to_fertilizer);
    // println!("{}", fertilizer_to_water.get_destination(53));
    // dbg!(&seeds);

    let maps=[&seed_to_soil,
    &soil_to_fertilizer,
    &fertilizer_to_water,
    &water_to_light,
    &light_to_temperature,
    &temperature_to_humidity,
    &humidity_to_location];

    let result=(0..100_000_000).into_iter()
    .find(|location:&u64| {
        let mut last_value:u64=*location;
        let mut value:u64;
        for map in maps.into_iter().rev() {
            value=map.get_source(last_value);
            last_value=value;
        }
        let seed=last_value;
        for range in &seed_ranges {
            if range.start<=seed && seed<=range.start+range.step {
                return true;
           }
       }
       return false;
    }).unwrap();

    // dbg!(&results);
    // let result=results.iter().reduce(|el1, el2| cmp::min(el1, el2));
    dbg!(result);
    
    Ok(())
}

struct Map {
    name: String,
    output: String,
    ranges: Vec<Range>,
}
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {:?})", self.name, self.ranges)
    }
}

impl Map {
    fn init(input: &mut dyn Iterator<Item=String>)->Self {
        let line=input.next().unwrap();
        let (name,_)=line.split_once(" ").unwrap();
        let ranges:Vec<Range>=input
        .take_while(|line| !line.is_empty())
        .map(|line| Range::parse(&line))
        .collect();
        let output=line.split("-").nth(2).unwrap()
            .strip_suffix(" map:").unwrap()
            .to_string();
        Self{
            name: name.to_string(),
            output,
            ranges
        }
    }

    fn get_destination(&self, source:u64)-> u64{
        for range in &self.ranges {
            let start=range.source_start;
            if source>=start && source<start+range.step {
                let step=source-start;
                return range.dest_start+step;
            }
        }
        return source;
    }

    fn get_source(&self, destination:u64)-> u64{
        for range in &self.ranges {
            let start=range.dest_start;
            if destination>=start && destination<start+range.step {
                let step=destination-start;
                return range.source_start+step;
            }
        }
        return destination;
    }
}

struct Range {
    dest_start: u64,
    source_start: u64,
    step: u64,
}
impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(dest_start:{}, source_start:{}, range:{})", self.dest_start, self.source_start, self.step)
    }
}

impl Range {
    fn parse(input: &str)->Self {
        let mut values=input.split(' ')
            .map(|number| u64::from_str(number).unwrap());
        Self{
            dest_start: values.next().unwrap(),
            source_start: values.next().unwrap(),
            step: values.next().unwrap(), 
        }
    }
}
#[derive(Debug)]
struct SeedRange {
    start: u64,
    step: u64,
}
impl SeedRange {
    fn parse(input: &str)->Self {
        let mut values=input.split(' ')
            .map(|number| u64::from_str(number).unwrap());
        Self{
            start: values.next().unwrap(),
            step: values.next().unwrap(), 
        }
    }
    fn inside(&self, input: u64)->bool {
        return self.start<=input&&input<self.start+self.step;
    }
}