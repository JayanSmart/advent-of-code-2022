use common::read_lines;
use std::{env, usize};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Stockpile {
    piles: Vec<Pile>
}

impl Stockpile {

    pub fn new(piles: Vec<Pile>) -> Self {
        Self { piles }
    }

    pub fn piles(&self) -> &Vec<Pile> {
        &self.piles
    }

    pub fn get_pile(&self, i: usize) -> Option<&Pile> {
        self.piles.get(i)
    }

    pub fn get_pile_len(&self, i: usize) -> Option<usize> {
        self.piles.get(i)
            .and_then(|pile| Some(pile.items().len()))
    }

    pub fn add_item(&mut self, item: Item, pile: usize) {
        if let Some(stack) = self.piles.get_mut(pile) {
            stack.add_item(item.clone());
        }
    }

    pub fn add_items(&mut self, items: Vec<Item>, pile: usize) {
        if let Some(stack) = self.piles.get_mut(pile) {
            stack.add_items(items);
        }
    }

    pub fn initiate(&mut self, mut state: Vec<String>) {

        // this is the indexes of each stack. We need this to know how many stacks there are.
        let n = usize::from_str(
            state.pop().unwrap()
                .chars().last().unwrap()
                .to_string().as_str()
        ).unwrap();

        self.piles = vec![Pile::new(vec![]); n];

        while let Some(line) = state.pop() {
            self.process_line(line.chars().collect());
        }
    }

    fn process_line(&mut self, data: Vec<char>) {
        let n = data.len();
        let mut pile = 0;

        for i in (1..n).step_by(4) {
            if let Some(c) = &data.get(i) {
                if c.is_alphanumeric() {
                    self.add_item(Item::new(c.to_owned().clone()), pile);
                }
            }
            pile += 1;
        }
    }

    // Assumption: All moves are "valid" (will never try remove from empty pile)
    pub fn move_crate(&mut self, from: usize, to: usize) {
        if let Some(from_pile) = self.piles.get_mut(from) {
            if let Some(item) = from_pile.remove_item() {
                self.add_item(item.clone(), to);
            }
        }
    }

    pub fn move_stack(&mut self, from: usize, to: usize, count: usize) {
        if let Some(from_pile) = self.piles.get_mut(from) {
            if let Some(items) = from_pile.remove_items(count) {
                dbg!(&items);
                self.add_items(items, to);
            }
        }

    }

    pub fn get_top_item_lables(self) -> Vec<char> {
        let mut out: Vec<char> = Vec::new();
        for pile in self.piles() {
            if let Some(item) = pile.items().last() {
                out.push(item.label.clone());
            }
        }
        out
    }

}

#[derive(Clone, Debug)]
struct Pile {
    items: Vec<Item>
}

impl Pile {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item.clone());
    }

    pub fn remove_item(&mut self) -> Option<Item> {
        self.items.pop()
    }

    pub fn add_items(&mut self, mut items: Vec<Item>) {
        while let Some(item) = items.pop() {
            self.add_item(item);
        }
    }

    pub fn remove_items(&mut self, count: usize) -> Option<Vec<Item>> {
        let mut items: Vec<Item> = Vec::new();
        for _ in 0..count {
            items.push(self.items.pop().unwrap().clone());
        }

        Some(items)
    }
}


#[derive(Clone, Debug)]
struct Item {
    label: char,
}

impl Item {
    pub fn new(label: char) -> Self {
        Self { label }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let mut stockpile: Stockpile = Stockpile::new(vec![]);

    let mut start_state: Vec<String> = vec![];
    let mut process_rules = false;

    if let Ok(lines) = read_lines(file_path) {
        for l in lines {
            if let Some(line) = l.ok() {
                if process_rules {
                    // move <count> from <from> to <to>
                    let mut words = line.split_whitespace();
                    let _ = words.next(); // move
                    let count = words.next().unwrap().parse::<usize>().unwrap();
                    let _ = words.next(); // from
                    let from = words.next().unwrap().parse::<usize>().unwrap() - 1; //zero index
                    let _ = words.next(); // to
                    let to = words.next().unwrap().parse::<usize>().unwrap() - 1; //zero index
                    // for _ in 0..count {
                    //     stockpile.move_crate(from, to);
                    // }
                    stockpile.move_stack(from, to, count);

                } else {
                    if line == "" {
                        dbg!(&start_state);
                        stockpile.initiate(start_state.clone());
                        process_rules = true;
                        dbg!(&stockpile);
                    } else {
                        start_state.push(line);
                    }
                }
            }
        }
    }

    let top_string: String = stockpile.get_top_item_lables().into_iter().collect();
    println!("{}", top_string);
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_stockpile_initiate_one_crate_every_row() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("[A] [B] [C]".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        dbg!(&stockpile);
        assert_eq!(stockpile.piles.len(), 3);
    }

    #[test]
    fn test_stockpile_initiate_only_adds_to_correct_piles() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("[D]     [E]".to_string());
        start_state.push("[A] [B] [C]".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        dbg!(&stockpile);

        assert_eq!(stockpile.get_pile_len(0).unwrap(), 2);
        assert_eq!(stockpile.get_pile_len(1).unwrap(), 1);
        assert_eq!(stockpile.get_pile_len(2).unwrap(), 2);

    }

    #[test]
    fn test_stockpile_initiate_only_adds_to_correct_pile_only_one_pile_full() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("        [E]".to_string());
        start_state.push("        [D]".to_string());
        start_state.push("        [C]".to_string());
        start_state.push("        [B]".to_string());
        start_state.push("        [A]".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        dbg!(&stockpile);

        assert_eq!(stockpile.get_pile_len(0).unwrap(), 0);
        assert_eq!(stockpile.get_pile_len(1).unwrap(), 0);
        assert_eq!(stockpile.get_pile_len(2).unwrap(), 5);
    }

    #[test]
    fn test_stockpile_move_single_crate_from_one_pile_to_another() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("        [A]".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        dbg!(&stockpile);

        stockpile.move_crate(2, 0);

        dbg!(&stockpile);

        assert_eq!(stockpile.get_pile_len(0).unwrap(), 1);
        assert_eq!(stockpile.get_pile_len(1).unwrap(), 0);
        assert_eq!(stockpile.get_pile_len(2).unwrap(), 0);
    }

    #[test]
    fn test_stockpile_move_from_one_pile_to_another_inverts_pile_order() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("[B]        ".to_string());
        start_state.push("[A]        ".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        assert_eq!(stockpile.get_pile(0).unwrap().items().first().unwrap().label, 'A');
        assert_eq!(stockpile.get_pile(0).unwrap().items().last().unwrap().label, 'B');

        dbg!(&stockpile);

        stockpile.move_crate(0, 1);
        stockpile.move_crate(0, 1);

        dbg!(&stockpile);

        assert_eq!(stockpile.get_pile_len(0).unwrap(), 0);
        assert_eq!(stockpile.get_pile_len(1).unwrap(), 2);
        assert_eq!(stockpile.get_pile_len(2).unwrap(), 0);

        assert_eq!(stockpile.get_pile(1).unwrap().items().first().unwrap().label, 'B');
        assert_eq!(stockpile.get_pile(1).unwrap().items().last().unwrap().label, 'A');
    }

    #[test]
    fn test_stockpile_move_stack_preserves_order() {
        let mut stockpile = Stockpile::new(vec![]);

        let mut start_state: Vec<String> = vec![];
        start_state.push("[B]        ".to_string());
        start_state.push("[A]        ".to_string());
        start_state.push(" 1   2   3".to_string());

        stockpile.initiate(start_state);

        assert_eq!(stockpile.get_pile(0).unwrap().items().first().unwrap().label, 'A');
        assert_eq!(stockpile.get_pile(0).unwrap().items().last().unwrap().label, 'B');

        dbg!(&stockpile);

        stockpile.move_stack(0, 1, 2);

        dbg!(&stockpile);

        assert_eq!(stockpile.get_pile_len(0).unwrap(), 0);
        assert_eq!(stockpile.get_pile_len(1).unwrap(), 2);
        assert_eq!(stockpile.get_pile_len(2).unwrap(), 0);

        assert_eq!(stockpile.get_pile(1).unwrap().items().first().unwrap().label, 'A');
        assert_eq!(stockpile.get_pile(1).unwrap().items().last().unwrap().label, 'B');
    }

}