use aoc_2021::MapWindowsExt;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Seek},
    time::Instant,
};

fn main() -> Result<()> {
    let start_time = Instant::now();
    let file = File::open("input/day01.txt")?;
    let mut reader = BufReader::new(file);
    println!("Part 1 => {}", run(&mut reader, 1));
    reader.rewind()?;
    println!("Part 2 => {}", run(&mut reader, 3));
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Took {} microseconds to run", duration.as_micros());
    Ok(())
}

/// Processes the input to count how many depth increases there are in the sums
/// of a given sliding window size (part 1 = 1, part 2 = 3).
fn run(reader: impl BufRead, window_size: usize) -> u32 {
    reader
        .lines() // iterator over the lines of the reader
        .filter_map(|elem| {
            let line = elem.ok()?;
            let int = line.trim().parse().ok()?;
            Some(int)
        }) // check whether the line is valid and parse as an integer if we can
        .map_windows(window_size, |elems: &[u32]| elems.into_iter().sum()) // for each group of window_size we want to sum them
        .map_windows(2, |elems: &[u32]| (elems[1] > elems[0]) as u32) // emit 1 if the element is greater than previous, 0 otherwise
        .sum() // sum all the 1's we've emitted
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example_part_1() {
        const INPUT: &[u8] = r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "#
        .as_bytes();
        const EXPECTED: u32 = 7;
        let calculated = run(INPUT, 1);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_example_part_2() {
        const INPUT: &[u8] = r#"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
            "#
        .as_bytes();
        const EXPECTED: u32 = 5;
        let calculated = run(INPUT, 3);
        assert_eq!(calculated, EXPECTED);
    }
}
