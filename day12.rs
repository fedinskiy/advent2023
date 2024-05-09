use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
// use std::fmt;
use std::str::FromStr;

fn main() -> std::io::Result<()> {

    let row=Row::parse("???.### 1,1,3");
    assert_eq!(row.groups,vec![1,1,3]);
    assert_eq!(vec![1,1,3], get_groups(&['#','.','#','.','#','#','#']));

    
    assert_eq!(8, generate_possible_solutions(&row.conditions).len());

    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let sum:usize = lines
    .map(|line| {
        Row::parse(&line)
        })
     .map(|row| process_row(&row))
     .fold(0, |acc, x| acc + x);
    println!("{sum}");
    Ok(())
}

struct Row {
    conditions: Vec<char>,
    groups: Vec<u8>
}

impl Row {
    fn parse(source: &str) -> Self {
        let (springs,numbers)=source.split_once(" ").unwrap();
        let conditions=springs.chars().collect();
        let groups=numbers
            .split(',')
            .map(|number| u8::from_str(number))
            .flatten()
            .collect();
        Row{groups, conditions}
    }
}

fn process_row(row: &Row) -> usize {
    let all=generate_possible_solutions(&row.conditions);
    // show(&all);
    let filtered=filter_solutions(all, &row.groups);
    // println!("Soultions for {:?} {:?}", row.conditions, row.groups);
    // show(&filtered);
    return filtered.len();
}

fn generate_possible_solutions(mask: &[char]) -> Vec<Vec<char>> {
    let broken=mask.iter().filter(|&&symbol| symbol=='?').count();
    let upper_solution_limit=2_usize.pow(broken.try_into().unwrap());
    let solution: Vec<char>=Vec::from(mask);
    // println!("{solution:?}");
    let mut solutions = vec![solution; upper_solution_limit];
    // show(&solutions);
    let mut period_length=1;
    for (position, &symbol) in mask.iter().enumerate() {
        if symbol=='?' {
            for (i, current) in solutions.iter_mut().enumerate() {
                if ((i/period_length)%2)==0 {
                    current[position]='.'
                } else {
                    current[position]='#'
                }
            }
            period_length*=2;
        }
    }
    // show(&solutions);
    solutions
}

fn filter_solutions(solutions: Vec<Vec<char>>, groups: &Vec<u8>) -> Vec<Vec<char>> {
    solutions.into_iter()
        .filter(|solution| &get_groups(solution)==groups)
        .collect()
}

fn get_groups(source: &[char])-> Vec<u8> {
    let mut result=Vec::new();
    let mut current_count=0;
    for &symbol in source {
        if symbol=='#' {
            current_count+=1;
        } else if symbol=='.' {
            if current_count!=0 {
                result.push(current_count);
            }
            current_count=0;
        }
    }
    if current_count!=0 {
        result.push(current_count);
    }
    return result;
}
fn show(source: &Vec<Vec<char>>) -> (){
    for line in source {
        println!("{line:?}");
    }
}

fn possible_solution(record: &[char], solution: &[char])-> bool {
    assert_eq!(record.len(), solution.len());
    for i in 0..record.len() {
        let still_possible:bool;
        let spring=record[i];
        if spring=='?' {
            still_possible = true;
        } else {
            still_possible=spring==solution[i];
        }
        if !still_possible {
            return false;
        }
    }
    return true;
}