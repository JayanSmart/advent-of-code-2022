use std::cmp::Ordering;
use std::env;
use std::process::exit;

use common::read_lines;

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
                        println!(
                            "Add Elf {}, with {} calories of food",
                            current.id, current.calories
                        );
                        elves.push(current.clone());
                        current = Elf::new(current.id + 1, 0);
                    } else {
                        current.add_calories(cal.parse::<i32>().unwrap());
                    }
                }
                None => print!("Found Nothing!"),
            }
        }
    }

    if elves.len() == 0 {
        println!("No elves on this trip!");
        exit(0)
    }

    println!(
        "The elf with the most food has {:?} calories",
        calculate_best_elf(&elves).calories
    );

    let best_elves = calculate_best_3_elves(&elves);
    println!(
        "The top 3 elves have {:?} calories",
        best_elves.iter().map(|x| x.calories).sum::<i32>()
    )
}

fn calculate_best_elf(elves: &Vec<Elf>) -> &Elf {
    elves.iter().max().unwrap()
}

fn calculate_best_3_elves<'a>(elves: &Vec<Elf>) -> Vec<Elf> {
    let mut sorted = elves.clone();
    sorted.sort_by_key(|x| -x.calories);
    sorted[0..3].to_vec()
}
