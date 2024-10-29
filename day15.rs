use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::str::FromStr;


fn main() -> std::io::Result<()> {
    assert_eq!(hash_str("HASH"), 52);
    assert_eq!(hash_str("rn"), 0);
    println!("start");
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines:Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();
    let steps:Vec<String>=lines.iter()
        .flat_map(|line| line.split(',').map(str::to_owned))
        .collect();
    println!("{steps:?}");
    let mut storage=Lenses::new();
    for step in steps.iter() {
        storage.perform(&step);
    }
    let sum = storage.sum();
    println!("{sum:?}");
    // debug(&platform,2);
    // debug(&platform,9);
    Ok(())
}

fn hash(input:&[u8]) -> usize  {
    let mut current:usize=0;
    for byte in input {
        current+=*byte as usize;
        current*=17;
        current%=256;
    }
 
    return current
}

fn hash_str(input:&str) -> usize  {
    hash(input.as_bytes())
}

#[derive(Clone)]
struct Lens {
    label:String,
    focal_length:u8
}

impl Lens {
    fn label(&self)->&str{
        &self.label
    }
}

struct Lenses {
    boxes: [Vec<Lens>; 256]
}

enum Action {
    Add,
    Remove,
}

impl Lenses {
    fn new() -> Self {
        Lenses{boxes:[const{Vec::new()}; 256]}
    }
    fn remove(&mut self, label:&str) {
        let hash=hash_str(label);
        let current_box=&self.boxes[hash];
        let mut updated=Vec::new();
        for lens in current_box.iter() {
            if lens.label!=label {
                updated.push(lens.clone());
            }
        }
        self.boxes[hash]=updated;
    }

    fn add(&mut self, new_lens:Lens) {
        let hash=hash_str(&new_lens.label);
        let current_box=&self.boxes[hash];
        let mut updated=Vec::new();
        let mut replaced=false;
        for lens in current_box.iter() {
            if lens.label!=new_lens.label{
                updated.push(lens.clone());
            } else {
                replaced=true;
                updated.push(new_lens.clone());
            }
        }
        if !replaced {
            updated.push(new_lens.clone());
        }
        self.boxes[hash]=updated;
    }

    fn perform(&mut self, step: &str) {
        let mut action=None;
        let mut label=String::new();
        let mut text_length=' ';
        for symbol in step.chars() {
            match symbol {
                '-' => action=Some(Action::Remove),
                '=' => action=Some(Action::Add),
                _=> { 
                    match action {
                        //part before action is label
                        None=> label.push(symbol),
                        //part after action is value
                        Some(_)=> text_length=symbol
                    }
                }
            }
        }
        match action {
            Some(Action::Remove)=>self.remove(&label),
            Some(Action::Add)=> {
                let length:u8=text_length.to_digit(10).unwrap().try_into().unwrap();
                self.add(Lens{  label:label,
                                focal_length: length})
            },
            None=> panic!("Unknow action in {step}")
        }
        // let bytes=step.as_bytes();
        // let label:[u8;2]=*bytes.first_chunk::<2>().unwrap();
        // let action=bytes[2];
        
        // match action {
        //     b'-' => self.remove(label),
        //     b'=' => {
        //         let text_length=&step[3..4];
        //         let length:u8=u8::from_str(text_length).expect(&("This value should be parsed: ".to_owned()+text_length));
        //         self.add(Lens{   label:label,
        //                             focal_length: length})
        //     },
        //     _ => panic!("Unknow value: {action} in {step}")
        // }
    }

    fn sum(&self)->usize {
        let mut power=0;
        for (number,current_box) in self.boxes.iter().enumerate() {
            let box_number=number+1;
            for (lens_number, lens) in current_box.iter().enumerate() {
                let slot=lens_number+1;
                let lens_power=box_number*slot*(lens.focal_length as usize);
                // println!("{}: {box_number} (box {number}) * {slot} (slot) * {} (focal length)",
                //     lens.label(),                                           lens.focal_length
                // );
                power+=lens_power;
            }
        }
        power
    }
}