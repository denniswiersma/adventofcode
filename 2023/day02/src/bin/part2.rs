fn readlines(input: &str) -> Vec<String> {
    input.split("\n").map(|line| String::from(line)).collect()
}

fn is_possible(line: String) -> u32 {
    let mut best_red = 0;
    let mut best_green = 0;
    let mut best_blue = 0;

    let parts: Vec<String> = line.split(":").map(|part| String::from(part)).collect();

    let hands: Vec<String> = parts[1].split(";").map(|part| String::from(part)).collect();

    for hand in hands {
        let cubes: Vec<String> = hand.split(",").map(|part| String::from(part)).collect();

        for cube in cubes {
            let cube_parts: Vec<String> = cube
                .split_whitespace()
                .map(|part| String::from(part))
                .collect();
            let ncube = cube_parts[0].parse::<u32>().unwrap();
            let cube_col = &cube_parts[1];

            if cube_col == "red" && ncube > best_red {
                best_red = ncube;
            } else if cube_col == "green" && ncube > best_green {
                best_green = ncube;
            } else if cube_col == "blue" && ncube > best_blue {
                best_blue = ncube;
            }
        }
    }
    best_red * best_green * best_blue
}

fn part2(input: &str) -> u32 {
    let lines = readlines(input);

    let mut total = 0;

    for line in lines {
        if line != String::from("") {
            let p = is_possible(line);
            total += p;
        }
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part2(input), 2286);
    }
}
