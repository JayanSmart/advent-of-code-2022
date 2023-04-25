use std::{env, path::Path};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

#[derive(Eq, Clone)]
struct Elf {
    id: i32,
    calories: i32,
}

impl Elf {
    pub fn new(id: i32, calories: i32) -> Self {
        Self { id, calories }
    }

    pub fn add_calories(&mut self, meal: i32) {
        self.calories = self.calories + meal;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn calories(&self) -> i32 {
        self.calories
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut elves: Vec<Elf> = vec![];

    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut current: Elf = Elf::new(0, 0);
        for line in lines {
            match line.ok() {
                Some(cal) => {
                    if cal == "" {
                        println!("Add Elf {}, with {} calories of food", current.id, current.calories);
                        elves.push(current.clone());
                        current = Elf::new(current.id + 1, 0);
                    } else {
                        current.add_calories(cal.parse::<i32>().unwrap());
                    }
                }
                None => print!("Found Nothing!")
            }
        }
    }

    if elves.len() == 0 {
        println!("No elves on this trip!");
        exit(0)
    }

    println!("The elf with the most food is: {:?}", calculate_best_elf(elves));



}

fn calculate_best_elf(elves: Vec<Elf>) -> i32 {
    elves.iter().max().unwrap().calories
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}