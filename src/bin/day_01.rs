// ##############
// ### PART 1 ###
// ##############

fn part1() -> u32 {
    let input = std::fs::read_to_string("data/day_01.txt").unwrap();
    input.trim().split("\r\n").map(|s| first_digit(s) * 10 + last_digit(s)).sum()
}

fn first_digit(s: &str) -> u32 {
    s.chars().find_map(|c| c.to_digit(10)).unwrap()
}

fn last_digit(s: &str) -> u32 {
    s.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
}

// ###############
// ### PART 2 ####
// ###############

const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn part2() -> u32 {
    let _input = parse_input();
    0
}

// Want to parameterize the way we iterate over characters, because we need to scan through the string we're searching in forward/reverse
// order, and we need this order to match the way we scan through the strings we're trying to match against.  Can we use char (unicode scalar) iterators
// to maintain the state of where we're currently scanning in the string?

// I'm imagining that some driver struct will maintain a vec of current scanners and a vec of next gen scanners.  The driver will iterate through
// chars in the string we're searching in whatever order our parameterized thing determines, and decide which scanners advance on to the next generation.
// The driver should also early exit if any of the scanners indicate that a match has occurred.  At the start of each generation we should add base
// scanners to the generation (since at every character we want to start a search).
fn first_digit_2(s: &str) -> u32 {
    find_digit(s, |s: &str| { s.chars().peekable() })
}

fn last_digit_2(s: &str) -> u32 {
    // Well, crap.  Reversing results in the `Rev` type in the type sig.
    find_digit(s, |s: &str| { s.chars().rev().peekable() })
}

fn find_digit(s: &str, str_to_iter: fn(&str) -> std::iter::Peekable<std::str::Chars>) -> u32 {
    let mut cur_scanners = Vec::new();
    let mut nxt_scanners = Vec::new();

    loop {
        for (value, string) in NUMBERS.iter().enumerate() {
            cur_scanners.append
        }
    }
}

// ############
// ### MAIN ###
// ############

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

// #############
// ### TESTS ###
// #############

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 54953);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
