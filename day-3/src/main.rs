use std::env;
use common::read_lines;

//let result =

#[derive(Clone, Debug)]
struct Rucksack {
    content: (String, String)
}

impl Rucksack {
    pub fn new(total: String) -> Self {
        let split = total.split_at( total.len()/2);
        let content = (String::from(split.0), String::from(split.1));
        Self { content }
    }

    fn get_common_item(self) -> Option<char> {
        self.content.0.chars().find(|&c| self.content.1.contains(c))
    }
}

fn get_item_score(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    c as u32 - 96
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut total_score: u32 = 0;
    let mut rucksacks: Vec<Rucksack> = vec![];
    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            match line.ok() {
                Some(line) => {
                    let current = Rucksack::new(line.clone());
                    rucksacks.push(current.clone());
                    let incorrect_item = current.get_common_item().expect("No common item found");
                    total_score += get_item_score(incorrect_item);
                }
                None => {}
            }
        }

        println!("Number of rucksacks: {:?}", rucksacks.len());
        println!("Total score: {:?}", total_score);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_item_converts_to_correct_score() {
        // Check the 4 boundaries as the input guarantees all characters are within these limits.
        assert_eq!(get_item_score('a'), 1);
        assert_eq!(get_item_score('z'), 26);
        assert_eq!(get_item_score('A'), 27);
        assert_eq!(get_item_score('Z'), 52);

    }
}