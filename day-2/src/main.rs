use std::env;

enum OpponentKey {
    A,
    B,
    C,
}

enum YourKey {
    X,
    Y,
    Z,
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
                Some(strat) => load_strat(strat),
                None => print!("Found Nothing!"),
            }
        }
    }

    if elves.len() == 0 {
        println!("No elves on this trip!");
        exit(0)
    }
}
