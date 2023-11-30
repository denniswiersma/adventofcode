use std::fs;

fn readlines(filename: &str) -> Vec<[u32;4]> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map()
        .collect()
}

fn line_to_array(line: String) -> [u32;4] {
    let mut ranges: [u32;4];
    for (i, range) in line.split(",").enumerate().step_by(2) {
        let parts: Vec<u32> = range.split("-").collect();
        ranges[i] = parts[0];
        ranges[i+1] = parts[1];
    }
    ranges
}

fn main() {
    println!("Hello, world!");
}
