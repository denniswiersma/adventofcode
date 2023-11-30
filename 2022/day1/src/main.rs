use std::fs;

fn main() {
    let lines = read_lines("input/input.txt");

    let mut totals_per_elf: Vec<u32> = lines.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    let elf_with_most = totals_per_elf.iter()
        .max()
        .unwrap();

    println!("part 1\ncalories of elf with most: {:?}", elf_with_most);

    totals_per_elf.sort_by(|a, b| b.cmp(a));
    let top_three = &totals_per_elf[0..3];

    let top_three_total: u32 = top_three.iter()
        .sum();


    println!("part 2\ncalories of top three elves: {:?}", top_three_total)
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
