use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_read_lines_returns_err_result_on_no_file() {
        assert_eq!(read_lines("FileDoesNotExist").is_err(), true);
    }

    #[test]
    fn test_read_lines_returns_result_on_found_file() {
        assert_eq!(read_lines("input.txt").is_ok(), true);
    }

    #[test]
    fn test_read_lines_ok_result_is_iterable() {
        for line in read_lines("input.txt").unwrap() {
            assert_eq!(line.is_ok(), true)
        }
    }

    #[test]
    fn test_read_lines_ok_result_contains_5_lines() {
        let mut count = 0;
        for _ in read_lines("input.txt").unwrap() {
            count += 1;
        }
        assert_eq!(count, 5)
    }
}
