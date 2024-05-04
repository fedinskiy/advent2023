use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file=File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);

    let mut galaxies: Vec<Coordinates>=Vec::new();
    let mut last_column=0;
    let mut last_row=0;
    for (row, line) in lines.enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol=='#' {
                galaxies.push(Coordinates{row,column});
            }
        }
        last_column=line.len()-1;
        last_row=row;
    }
    // println!("{:?}", galaxies);
    println!("last_row:{last_row}, last_column: {last_column}");
    let mut empty_column=vec![true;last_column+1];
    let mut empty_row=vec![true; last_row+1];

    for galaxy in &galaxies {
        empty_column[galaxy.column]=false;
        empty_row[galaxy.row]=false;
    }
    let age_coefficient=1000000-1;
    for mut galaxy in &mut galaxies {
        let mut empty_rows: usize =0;
        for i in 0..galaxy.row {
            if empty_row[i] {
                empty_rows+=1;
            }
        }
        let mut empty_columns: usize =0;
        for i in 0..galaxy.column {
            if empty_column[i] {
                empty_columns+=1;
            }
        }
        galaxy.row=galaxy.row+empty_rows*age_coefficient;
        galaxy.column=galaxy.column+empty_columns*age_coefficient;
    }

    // println!("{:?}", galaxies);
    println!("{:?}", get_all_distances(&galaxies));
    // println!("{empty_column:?} {empty_row:?}");
    Ok(())
}

fn get_all_distances(galaxies: &Vec<Coordinates>)-> usize {
    let mut distances=0;
    let mut unprocessed_galaxies:HashSet<&Coordinates>=galaxies.iter().collect();
    for galaxy in galaxies {
        unprocessed_galaxies.remove(galaxy);
        for other_galaxy in &unprocessed_galaxies {
                distances+=galaxy.get_distance(other_galaxy);
        }
    }
    return  distances
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinates {
    column: usize,
    row: usize
}

impl Coordinates {
    fn get_distance(&self, other: &Coordinates) -> usize {
        let vertical = if self.row>other.row {self.row-other.row} else {other.row-self.row};
        let horizontal = if self.column>other.column {self.column-other.column} else {other.column -self.column};
        vertical+horizontal
    }
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row+1, self.column+1)
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