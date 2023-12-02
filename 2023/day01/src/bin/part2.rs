fn readlines(input: &str) -> Vec<String> {
    input.split("\n").map(|line| String::from(line)).collect()
}

fn letters_to_digits(line: String) -> String {
    let written_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut letters_done = String::from("");

    for letter in line.chars() {
        letters_done.push(letter);
        for (i, digit) in written_digits.iter().enumerate() {
            letters_done = letters_done.replace(
                digit,
                format!("{}{}", i + 1, letters_done.chars().last().unwrap()).as_str(),
            );
        }
    }

    letters_done
}

fn find_first_digit(line: String) -> Option<u32> {
    for c in line.chars() {
        if c.is_digit(10) {
            return Some(c as u32 - 48);
        }
    }
    None
}

fn part2(input: &str) -> u32 {
    let mut lines = readlines(input);
    lines.pop();

    let mut total = 0;

    for mut line in lines {
        line = letters_to_digits(line.clone());
        let first_digit = find_first_digit(line.clone()).unwrap();
        let last_digit = find_first_digit(line.chars().rev().collect()).unwrap();
        let full_digit = first_digit * 10 + last_digit;
        total += full_digit;
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
