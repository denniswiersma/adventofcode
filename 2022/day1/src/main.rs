use std::fs;

fn main() {
    let lines = read_lines("input/input.txt");

    let totals_per_elf: Vec<u32> = lines.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    let elf_with_most = totals_per_elf.iter()
        .max()
        .unwrap();

    println!("{:?}", elf_with_most)
}

fn read_lines(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|chunk| {
            chunk.lines()
                .filter_map(|line| line.parse::<u32>().ok())
                .collect()
        })
        .collect()
}
