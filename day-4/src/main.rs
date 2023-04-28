use common::read_lines;
use std::env;
use std::str::FromStr;

pub fn is_fully_contained(x: &Vec<i32>, y: &Vec<i32>) -> bool {
    if x.len() == 1 {
        return y.contains(x.first().unwrap());
    }

    if y.len() == 1 {
        return x.contains(y.first().unwrap());
    }

    // check if y inside x
    if x.first().unwrap() <= y.first().unwrap() && x.last().unwrap() >= y.last().unwrap() {
        return true;
    // check if x inside y
    } else if x.first().unwrap() >= y.first().unwrap() && x.last().unwrap() <= y.last().unwrap() {
        return true;
    }

    false
}

pub fn is_partially_contained(x: &Vec<i32>, y: &Vec<i32>) -> bool {

    return x.iter().any(|x| y.contains(x));

}

fn str_to_range(text: &str) -> Vec<i32> {
    let mut splits = text.split('-');
    (FromStr::from_str(splits.next().unwrap()).unwrap()
        ..=FromStr::from_str(splits.next().unwrap()).unwrap())
        .collect()
}

fn compute_line(line: String) -> (bool, bool) {
    let mut splits = line.split(',');
    let sections = (splits.next().unwrap(), splits.next().unwrap());

    let ranges = (&str_to_range(&sections.0), &str_to_range(&sections.1));

    let full = is_fully_contained(
        ranges.0, ranges.1
    );

    let partial = is_partially_contained(
        ranges.0, ranges.1
    );

    (full, partial)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let mut total_contained: i32 = 0;
    let mut part_contained: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            match line.ok() {
                Some(line) => {
                    dbg!(&line);
                    let result = compute_line(line);
                    if result.0 {
                        total_contained += 1;
                    }

                    if result.1 {
                        part_contained += 1;
                    }
                }
                None => {}
            }
        }
    }

    println!("Total pairs where fully contained: {}", total_contained);
    println!("Total pairs where partially contained: {}", part_contained);
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fist_vec_contains_second_vec() {
        let vec1: Vec<i32> = (1..5).collect();
        let vec2: Vec<i32> = (2..4).collect();

        assert_eq!(is_fully_contained(&vec1, &vec2), true)
    }

    #[test]
    fn test_second_vec_contains_first_vec() {
        let vec1: Vec<i32> = (1..5).collect();
        let vec2: Vec<i32> = (2..4).collect();

        assert_eq!(is_fully_contained(&vec2, &vec1), true)
    }

    #[test]
    fn test_vec_contains_itself() {
        let vec: Vec<i32> = (1..5).collect();

        assert_eq!(is_fully_contained(vec.clone().as_ref(), &vec), true)
    }

    #[test]
    fn test_same_start_contains() {
        let vec1: Vec<i32> = (1..5).collect();
        let vec2: Vec<i32> = (1..4).collect();

        assert_eq!(is_fully_contained(&vec1, &vec2), true)
    }

    #[test]
    fn test_same_end_contains() {
        let vec1: Vec<i32> = (1..5).collect();
        let vec2: Vec<i32> = (2..5).collect();

        assert_eq!(is_fully_contained(&vec1, &vec2), true)
    }

    #[test]
    fn test_does_not_contain() {
        let vec1: Vec<i32> = (1..5).collect();
        let vec2: Vec<i32> = (2..6).collect();

        assert_eq!(is_fully_contained(&vec1, &vec2), false)
    }

    #[test]
    fn test_process_line_fully_contained() {
        assert_eq!(compute_line("1-5,2-4".to_string()), (true, true))
    }

    #[test]
    fn test_process_line_no_overlap() {
        assert_eq!(compute_line("1-5,6-10".to_string()), (false, false))
    }

    #[test]
    fn test_process_line_partially_contained() {
        assert_eq!(compute_line("1-5,2-6".to_string()), (false, true))
    }

    #[test]
    fn test_process_line_same_start_and_end_value() {
        assert_eq!(compute_line("64-64,12-63".to_string()), (false, false))
    }
}
