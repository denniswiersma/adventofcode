fn readlines(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(|line| String::from(line))
        .collect()
}

fn find_first_digit(line: String) -> Option<u32> {
    for c in line.chars() {
        if c.is_digit(10) {
            return Some(c as u32 - 48)
        }
    }
    None
}

fn part1(input: &str) -> u32 {
    let mut lines = readlines(input);
    lines.pop();

    let mut total = 0;

    for line in lines {
        let first_digit = find_first_digit(line.clone()).unwrap();
        let last_digit = find_first_digit(line.chars().rev().collect()).unwrap();
        let full_digit = first_digit * 10 + last_digit;
        total += full_digit;
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(input), 142);
    }
}
