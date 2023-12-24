use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    assert_eq!(21, gcd(1071,462));
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok);

    let mut common_steps=Steps::parse(&lines.next().unwrap());
    let nodes:HashMap<String, Node>=lines
    .skip(1)
    .map(|line| Node::parse(line))
    .map(|node| {
        (node.name.clone(), node)
    })
    .collect();
    
    let mut start_nodes:Vec<&Node>=nodes.keys()
    .filter(|key| &key.chars().nth(2).unwrap()==&'A')
    .map(|key| {
        nodes.get(key).unwrap()
    })
    .collect();
    println!("{:?}",start_nodes);

    let results:Vec<usize>=start_nodes
    .iter()
    .map(|&start_node|{
        let mut current:&Node=start_node;
        let mut steps=common_steps.clone();
        while !current.is_finish() {
                let next = match steps.next() {
                    Step::Left => nodes.get(&current.left),
                    Step::Right => nodes.get(&current.right),
                };
                assert!(next.is_some(), "failure on step {} for node {}", steps.count, current.name);
                current=next.unwrap();
        }
        return steps.count;
    })
    .collect();

    println!("{:?}",results);

    let mut results_lcm = results[0];
    for i in 1..results.len() {
        let _lcm=lcm(results_lcm, results[i]);
        results_lcm=_lcm;
        println!("{:?}",results_lcm);
    }
    println!("{:?}",results_lcm);
    Ok(())
    
}
fn lcm(_1:usize, _2:usize)->usize {
    let gcd=gcd(_1, _2);
    dbg!(_1,_2,gcd);
    return (_2/gcd)*_1;
}
fn gcd(_1:usize, _2:usize)->usize {
    let (mut first, mut second) = (_1, _2);
    while (first!=second) {
        if first>second {
            first=first-second;
        } else {
            second=second-first;
        }
    }
    return first;
}
fn finish(current_nodes: &Vec<&Node>)->bool {
    current_nodes.iter().all(|node| node.is_finish())
}
#[derive(Debug, PartialEq, Clone)]
enum Step {
    Left,
    Right
}
#[derive(Clone)]
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

    fn is_finish(&self)-> bool {
        self.name.chars().nth(2).unwrap()=='Z'
    }
}