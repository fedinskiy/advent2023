use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    //let file = File::open("test.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result:u32 = 0;
    while let Ok(1..) = reader.read_line(&mut line) {
        let first_digit=ForwardParser::create().get_first_digit(&mut line.chars());
        let last_digit=BackwardParser::create().get_first_digit(&mut line.chars().rev());
//        println!("{} {}", first_digit,last_digit);
        result+=first_digit*10+last_digit;
        line.clear();
    }
    println!("{}", result);
    Ok(())
}
struct Parser {
    word: String
}

struct ForwardParser {
    common: Parser,
}

impl Parse for ForwardParser {
    fn create()->Self {
        Self {
            common: Parser {
            word: String::new()}
        }
    }
    fn get_word(&self)->&String{
        &self.common.word
    }
    fn drop_invalid_parts(&mut self) {
        self.common.word.remove(0);
        () //ignore the removed char
    }
    fn is_promising(&self, str: &str)->bool {
        str.starts_with(&self.common.word)
    }
    fn add(&mut self, ch: char) {
        self.common.word.push(ch);
    }
}
struct BackwardParser {
    common: Parser
}
impl Parse for BackwardParser {
    fn create()->Self {
        Self {
            common: Parser {
            word: String::new()}
        }
    }
    fn get_word(&self)->&String{
        &self.common.word
    }
    fn drop_invalid_parts(&mut self) {
        self.common.word.pop();
        () //ignore the removed char
    }
    fn is_promising(&self, str: &str)->bool {
        str.ends_with(&self.common.word)
    }
    fn add(&mut self, ch: char) {
        self.common.word.insert(0, ch);
    }
}

const DIGITS:[&str; 10]=["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
trait Parse {
    fn create()->Self;
    fn get_word(&self)->&String;
    fn drop_invalid_parts(&mut self);
    fn is_promising(&self, str: &str)->bool;
    fn add(&mut self, ch: char);
    
    fn get_first_digit(&mut self,chars: &mut dyn Iterator<Item=char>)->u32 {
        while let Some(ch)=chars.next() {
            if ch.is_ascii_digit() {
                return ch.to_digit(10).unwrap_or(0);
            } else {
                self.add(ch);
                let word=self.get_word();
                let mut could_be=false;
                for digit in IntoIterator::into_iter(DIGITS).enumerate() {
                    let (position, name) = digit;
                    if word==name {
                        let number:u32 = position.try_into().unwrap(); // 0..10 should fit anywhere
                        return number;
                    } else {
                        could_be |= self.is_promising(name);
                 }
             }
             if !could_be && !word.is_empty() {
                    self.drop_invalid_parts();
                }
            }   
        }   
        return 0;
    }
}

