use std::fs;

fn read_lines(filename: &str, part: u8) -> Vec<RPSgame> {
    match part {
        1 => fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|l| line_to_game(l))
            .collect(),
        2 => fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|l| line_to_game_part2(l))
            .collect(),
        _ => panic!("Invalid part")
    }
}

fn line_to_game(line: &str) -> RPSgame {
    let split_line: Vec<char> = line.split(' ')
        .filter_map(|s| s.chars().next())
        .collect();


    let mut game: [Option<Shape>; 2] = [None, None];

    for (i, shape)in split_line.iter().enumerate() {
        let play = match shape {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Found incomplete match")
        };
        game[i] = Some(play);
    }

    let game: [Shape; 2] = [
        game[0].expect("Play not initialized"),
        game[1].expect("Play not initialized"),
    ];

    RPSgame { opponent: game[0], me: game[1] }
}

fn line_to_game_part2(line: &str) -> RPSgame {
    let split_line: Vec<char> = line.split(' ')
        .filter_map(|s| s.chars().next())
        .collect();

    let opponent = split_line[0];
    let outcome = split_line[1];

    let opponent_shape = match opponent {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Invalid opponent shape")
    };

    let my_hand = match (opponent_shape, outcome) {
        (Shape::Paper, 'X') | (Shape::Rock, 'Y') | (Shape::Scissors, 'Z') => Shape::Rock,
        (Shape::Scissors, 'X') | (Shape::Paper, 'Y') | (Shape::Rock, 'Z') => Shape::Paper,
        (Shape::Rock, 'X') | (Shape::Scissors, 'Y') | (Shape::Paper, 'Z') => Shape::Scissors,
        _ => panic!("Invalid shapes")
    };

    RPSgame { opponent: opponent_shape, me: my_hand }
}

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

struct RPSgame {
    opponent: Shape,
    me: Shape
}

fn calculate_match_score(game: RPSgame) -> u32 {
    let mut score = 0;

    use Shape::*;

    match game.me {
        Rock => score += 1,
        Paper => score += 2,
        Scissors => score += 3
    };


    match (game.opponent, game.me) {
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => score += 0, // opponent wins
        (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => score += 6, // I win
        _ => score += 3 // draw
    }

    score
}

fn main() {
    let games = read_lines("input/input.txt", 1);

    let mut total_score = 0;

    for game in games {
        total_score += calculate_match_score(game);
    };

    println!("part1\ntotal score: {}", total_score);

    let games = read_lines("input/input.txt", 2);

    let mut total_score = 0;

    for game in games {
        total_score += calculate_match_score(game);
    };

    println!("part2\ntotal score: {}", total_score);

}
