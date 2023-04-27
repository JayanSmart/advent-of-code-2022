use common::read_lines;
use std::env;

//let result =

#[derive(Clone, Debug)]
struct Rucksack {
    total_contents: String,
    containers: (String, String),
}

impl Rucksack {
    pub fn new(total_contents: String) -> Self {
        let split = total_contents.split_at(total_contents.len() / 2);
        let content = (String::from(split.0), String::from(split.1));
        Self {
            total_contents,
            containers: content,
        }
    }

    fn get_common_item(self) -> Option<char> {
        self.containers
            .0
            .chars()
            .find(|&c| self.containers.1.contains(c))
    }
}

#[derive(Clone, Debug)]
struct Party {
    elves: [Option<Rucksack>; 3],
}

impl Party {
    pub fn new() -> Self {
        let elves: [Option<Rucksack>; 3] = [None, None, None];
        Self { elves }
    }

    pub fn add_elf(&mut self, elf: Rucksack) -> Result<&str, &str> {
        for i in 0..self.elves.len() {
            // Find the first element of the party which is None
            if self.elves[i].is_none() {
                self.elves[i] = Some(elf.clone());
                return Ok("Elf Added");
            }
        }
        return Err("Party already full!");
    }

    pub fn find_identity_item(self) -> Result<char, &'static str> {
        if self.is_full() {
            return self.elves[0].as_ref().unwrap().total_contents.chars()
                .find(|&x| {
                    self.elves[1].as_ref().unwrap().total_contents.contains(x)
                        && self.elves[2].as_ref().unwrap().total_contents.contains(x)
                }).ok_or("Unable to find identity item");
        }

        Err(
            "Cannot find identity item of incomplete party. Party must contain 3 elves.",
        )
    }

    pub fn is_full(&self) -> bool {
        self.elves.iter().all(|x| x.is_some())
    }
}

fn get_item_score(c: &char) -> u32 {
    if c.is_uppercase() {
        return c.clone() as u32 - 38;
    }
    c.clone() as u32 - 96
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut total_score: u32 = 0;
    let mut total_party_score: u32 = 0;
    let mut parties: Vec<Party> = vec![];
    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut party = Party::new();
        for line in lines {
            match line.ok() {
                Some(line) => {
                    let rucksack = Rucksack::new(line.clone());
                    party.add_elf(rucksack.clone()).expect("Failed to add elf to party.");

                    let incorrect_item = &rucksack.get_common_item().expect("No common item found");
                    total_score += get_item_score(&incorrect_item);

                    if party.is_full() {
                        parties.push(party.clone());

                        let identity = party.find_identity_item().expect("No identity item found!");

                        total_party_score += get_item_score(&identity);

                        party = Party::new();
                    }

                }
                None => {}
            }
        }

        println!("Number of parties: {:?}", parties.len());
        println!("Total score: {:?}", total_score);
        println!("Total party score: {:?}", total_party_score);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_item_converts_to_correct_score() {
        // Check the 4 boundaries as the input guarantees all characters are within these limits.
        assert_eq!(get_item_score(&'a'), 1);
        assert_eq!(get_item_score(&'z'), 26);
        assert_eq!(get_item_score(&'A'), 27);
        assert_eq!(get_item_score(&'Z'), 52);
    }

    #[test]
    fn test_find_party_identity_item_success() {
        let mut party = Party::new();
        party
            .add_elf(Rucksack::new(
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            ))
            .expect("unexpected");
        party
            .add_elf(Rucksack::new(
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            ))
            .expect("unexpected");
        party
            .add_elf(Rucksack::new("PmmdzqPrVvPwwTWBwg".to_string()))
            .expect("unexpected");

        dbg!(&party);

        assert_eq!(party.find_identity_item().unwrap(), 'r')
    }

    #[test]
    fn test_find_party_errors_on_incomplete_party() {
        let mut party = Party::new();
        party
            .add_elf(Rucksack::new(
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            ))
            .expect("unexpected");
        party
            .add_elf(Rucksack::new(
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            ))
            .expect("unexpected");
        // Third party member is None

        dbg!(&party);

        assert_eq!(
            party.find_identity_item(),
            Err("Cannot find identity item of incomplete party. Party must contain 3 elves.")
        )
    }
}
