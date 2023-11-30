fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "put example input here";

        assert_eq!(part1(input), "put example output here");
    }
}
