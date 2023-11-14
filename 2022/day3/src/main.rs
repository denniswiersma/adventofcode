use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_duplicate(input: String) -> Option<char>{
    let split_point = input.chars().count() / 2;
    let (comp1, comp2) = input.split_at(split_point);

    for letter in comp1.chars() {
        if comp2.contains(letter) {
            return Some(letter)
        }
    }
    None
}

fn char_to_num(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u32) - 96, // since ASCII of 'a' is 97 and 'a' should equal 1
        'A'..='Z' => (letter as u32) - 38, // since ASCII of 'A' is 65 and 'A' should equal 27
        _ => 0,
    }
}

fn main() {
    let lines: Vec<String> = read_lines("input/input.txt");

    let mut duplicates: Vec<char> = Vec::new();
    for line in lines {
        duplicates.push(find_duplicate(line).unwrap())
    }

    let sum: u32 = duplicates
        .iter()
        .map(|dup| char_to_num(*dup))
        .sum();

    println!("{}", sum)
}