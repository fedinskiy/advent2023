use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::env;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok);

    let mut steps=Steps::parse(&lines.next().unwrap());
    let nodes:HashMap<String, Node>=lines
    .skip(1)
    .map(|line| Node::parse(line))
    .map(|node| {
        (node.name.clone(), node)
    })
    .collect();


    let mut current:&Node=nodes.get("AAA").unwrap();
    println!("{:?}",current);
    while current.name!="ZZZ" {
        let next = match steps.next() {
            Step::Left => nodes.get(&current.left),
            Step::Right => nodes.get(&current.right),
        };
        assert!(next.is_some(), "failure on step {} for node {}", steps.count, current.name);
        current=next.unwrap();
    }
    let mut result:usize = steps.count;
    println!("{:?}",result);
    Ok(())
    
}
#[derive(Debug, PartialEq)]
enum Step {
    Left,
    Right
}
struct Steps {
    steps: Vec<Step>,
    count: usize,
}
impl Steps {
    fn parse(source: &str) -> Self {
        let mut steps:Vec<Step>=Vec::with_capacity(source.len());
        for ch in source.chars() {
            steps.push(match ch {
                'L' => Step::Left,
                'R' => Step::Right,
                _ => panic!("unexpected step: {}", ch)
            })
        }
        Self {
            steps,
            count: 0
        }
    }
    fn next(&mut self) -> &Step {
        let next = &self.steps[self.count%self.steps.len()];
        self.count+=1;
        return next;

    }
}
#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String 
}

impl Node {
    fn parse (source: String)-> Self {
        Self {
            name: source[0..3].to_string(),
            left: source[7..10].to_string(),
            right: source[12..15].to_string(),
        }
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
}