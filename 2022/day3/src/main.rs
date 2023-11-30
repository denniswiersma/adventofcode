use std::{fs, collections::HashSet};

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_duplicate(input: &String) -> Option<char>{
    let split_point = input.chars().count() / 2;
    let (comp1, comp2) = input.split_at(split_point);

    for letter in comp1.chars() {
        if comp2.contains(letter) {
            return Some(letter)
        }
    }
    None
}

fn find_badge(rucksacks: Vec<String>) -> Vec<char> {
    let mut badges = Vec::new();

    for i in (0..rucksacks.len()).step_by(3) {
        let elf1 = &rucksacks[i];
        let elf2 = &rucksacks[i + 1];
        let elf3 = &rucksacks[i + 2];

        let mut checked = HashSet::new();


        for letter in elf1.chars() {
            if !checked.contains(&letter) && elf2.contains(letter) && elf3.contains(letter) {
                badges.push(letter);
                checked.insert(letter);
            }
        }
    }
    return badges;
}

fn char_to_num(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u32) - 96, // since ASCII of 'a' is 97 and 'a' should equal 1
        'A'..='Z' => (letter as u32) - 38, // since ASCII of 'A' is 65 and 'A' should equal 27
        _ => 0,
    }
}

fn main() {
    // part 1
    let lines: Vec<String> = read_lines("input/input.txt");

    let mut duplicates: Vec<char> = Vec::new();
    for line in &lines {
        duplicates.push(find_duplicate(line).unwrap())
    }

    let sum: u32 = duplicates
        .iter()
        .map(|dup| char_to_num(*dup))
        .sum();

    println!("part1: {}", sum);

    // part 2
    let badges = find_badge(lines);

    let sum: u32 = badges
        .iter()
        .map(|badge| char_to_num(*badge))
        .sum();

    println!("part2: {}", sum)

}
