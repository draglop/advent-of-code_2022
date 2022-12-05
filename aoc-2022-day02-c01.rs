// https://adventofcode.com/2022/day/2
// (part 1)

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
const SHAPES_COUNT: usize = 3;

#[derive(Clone, Copy)]
enum Result {
    Loss,
    Draw,
    Win,
}

/*
                |           opponent
                |   Rock    Paper   Scissors
    ----------------------------------------
        Rock    |   Draw    Loss    Win
    me  Paper   |   Win     Draw    Loss
        Scissors|   Loss    Win     Draw
*/
#[rustfmt::skip]
const RESULTS: [Result; SHAPES_COUNT * SHAPES_COUNT] = [
    Result::Draw, Result::Loss, Result::Win,
    Result::Win,  Result::Draw, Result::Loss,
    Result::Loss, Result::Win,  Result::Draw,
];

fn shape_get_opponent(c: char) -> Shape {
    return match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("unexpected opponent shape [{}]", c),
    };
}

fn shape_get_me(c: char) -> Shape {
    return match c {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("unexpected me shape [{}]", c),
    };
}

fn round_result(me: &Shape, opponent: &Shape) -> Result {
    let opponent_offset: usize = *opponent as usize;
    let me_offset: usize = (*me as usize) * SHAPES_COUNT;
    return RESULTS[opponent_offset + me_offset];
}

fn round_get(round: &str) -> (Shape, Result) {
    assert_eq!(round.len(), 3);
    assert_eq!(round.chars().nth(1).unwrap(), ' ');
    let opponent: Shape = shape_get_opponent(round.chars().nth(0).unwrap());
    let me: Shape = shape_get_me(round.chars().nth(2).unwrap());
    let result = round_result(&me, &opponent);
    return (me, result);
}

fn result_score(result: &Result) -> i32 {
    return match result {
        Result::Loss => 0,
        Result::Draw => 3,
        Result::Win => 6,
    };
}

fn shape_score(shape: &Shape) -> i32 {
    return match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
}

fn compute_score(list: &str) -> i32 {
    let mut score: i32 = 0;

    let lines: Vec<&str> = list.split('\n').collect();
    for line in lines {
        let (shape, result) = round_get(line);
        score += shape_score(&shape);
        score += result_score(&result);
    }

    return score;
}

fn main() {
    // example
    assert_eq!(15, compute_score("A Y\nB X\nC Z"));
    //// user puzzle input
    assert_eq!(8890, compute_score("A X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nB X\nA Z\nA X\nA X\nA X\nA X\nB X\nA X\nB X\nA Z\nA Z\nA X\nA X\nA Z\nA Z\nA Z\nA X\nA Z\nA X\nC Y\nC Z\nA X\nB X\nA X\nA X\nA X\nA Z\nB X\nA X\nB X\nC Y\nA X\nA Z\nA Y\nA X\nA Z\nA X\nA Z\nB X\nA X\nA Z\nB X\nA X\nA Z\nA X\nA Z\nA Z\nA X\nB X\nB X\nB X\nB Z\nA Z\nB Z\nA X\nA X\nA Z\nA X\nB Z\nB Y\nA Z\nB X\nA Y\nB X\nC X\nB Z\nB X\nC Y\nA Z\nA X\nA X\nC Y\nB X\nA X\nA Y\nA X\nA X\nA Y\nA Z\nC Z\nA Z\nB X\nB X\nA X\nB X\nA X\nA X\nB Y\nB X\nA Z\nB X\nB X\nA X\nB Z\nC Z\nA Z\nC Z\nB X\nB X\nA Z\nA X\nA X\nB X\nA Z\nB X\nA Z\nA X\nA X\nA X\nB X\nB X\nB X\nB X\nA X\nA Y\nA X\nB X\nA Z\nA X\nA X\nA X\nC Y\nA X\nB X\nA X\nA Z\nA X\nB X\nA Y\nB X\nC Z\nA Y\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nB X\nA X\nA X\nA Z\nA Z\nA X\nC Y\nB X\nB X\nA Z\nB X\nB Y\nC Y\nA X\nA X\nB Y\nA X\nB Y\nB X\nB X\nA X\nA Z\nB X\nB X\nA Y\nA X\nC Y\nC Y\nC Y\nA X\nB X\nA X\nA X\nA X\nB X\nB X\nC Y\nA X\nA Z\nA Z\nA X\nB X\nA Z\nA Z\nA X\nB X\nA X\nA Z\nA X\nC Z\nB X\nA X\nC Y\nC Y\nA Z\nB X\nB Z\nC Y\nB X\nA Y\nB Y\nA Z\nA X\nA X\nB X\nB Y\nA Z\nA X\nB X\nA X\nC X\nC Y\nA X\nB X\nA Z\nA X\nB X\nA X\nC Y\nC Y\nC Y\nA X\nA X\nA Z\nA X\nC Y\nA Z\nB Y\nA X\nC Y\nB Z\nB X\nB X\nA Z\nB X\nA Y\nC X\nA X\nB X\nA Z\nC Y\nA X\nA Z\nC Z\nA X\nC Y\nB X\nC Y\nC Z\nA Y\nA Z\nC Y\nA X\nA X\nA X\nA Y\nB X\nB X\nC Y\nC Y\nA X\nA Z\nC Y\nB Z\nB X\nC Z\nB X\nB X\nC Z\nA X\nA X\nB X\nB X\nA X\nA X\nB X\nB X\nB X\nC Y\nA X\nA X\nA X\nA X\nB X\nA X\nA X\nB X\nB Z\nA X\nB X\nA X\nC Y\nB Y\nA X\nB X\nA Y\nB Z\nB X\nA X\nB Z\nB X\nA Y\nA X\nB X\nA X\nA Z\nB X\nA X\nA X\nC Z\nA X\nB X\nA X\nB Y\nA Z\nA X\nB X\nA X\nA Z\nB X\nB X\nB Y\nB X\nA X\nC Y\nA X\nA X\nB X\nB Y\nA X\nA X\nA X\nC Z\nC Z\nB Y\nB X\nB Z\nA X\nA Y\nA X\nC Y\nB X\nA X\nA X\nA X\nA Z\nC Y\nA X\nA X\nC Y\nA X\nA X\nA X\nA Z\nA Y\nA X\nA Y\nC Z\nA Z\nA X\nB X\nA Z\nC Z\nC Y\nA Z\nA Z\nC Y\nC Z\nC Z\nA X\nB X\nB X\nB Z\nC Y\nA X\nA X\nA X\nA Z\nA X\nB X\nB X\nC X\nA X\nA X\nB X\nA Y\nA X\nA X\nB Y\nC Z\nA X\nA X\nA Y\nB X\nB X\nB X\nC Z\nA Z\nA Y\nB Y\nA X\nB X\nB X\nB X\nB Z\nA Y\nB Y\nA Z\nA Z\nC Z\nB X\nA X\nA Y\nA Y\nC Y\nB Y\nA X\nA X\nA X\nA X\nB Y\nA X\nA Y\nA X\nA Z\nB X\nB X\nA Y\nA X\nA Y\nA Y\nA Z\nB X\nB X\nA Z\nB X\nA X\nA Z\nA Z\nB X\nA Z\nA X\nA Y\nA Z\nA X\nA Z\nB Y\nA X\nC Y\nB X\nC X\nC Y\nB Z\nC Y\nC Y\nA X\nA X\nA Y\nB X\nC Z\nB X\nC Y\nA X\nA Z\nC Y\nA X\nA Z\nA Z\nB Y\nA X\nA X\nB X\nB Y\nA X\nA X\nB X\nA X\nB X\nA X\nB X\nA X\nB Y\nA Z\nB X\nB X\nB X\nB X\nA Y\nA X\nA Y\nA Y\nC Z\nB Z\nA X\nB Y\nA X\nC Z\nB X\nA X\nA X\nA X\nA X\nA X\nC Z\nC Z\nC Z\nA X\nC Y\nA X\nC X\nC Z\nA X\nB X\nA Z\nA Z\nA Z\nA X\nA Z\nA Z\nB Z\nA X\nB X\nA Z\nC Y\nA X\nA Z\nA Z\nA Z\nB Z\nB X\nA X\nB X\nB X\nB X\nB X\nB Y\nB X\nA X\nA X\nA Z\nA Z\nA Z\nB Y\nA X\nA X\nA Y\nB X\nB X\nA Z\nA X\nB X\nA X\nA X\nA Z\nC Y\nA X\nB X\nA X\nB X\nA X\nB X\nA Z\nB X\nA X\nA Z\nA Z\nB X\nA Y\nA X\nA Z\nC Y\nA X\nA X\nA X\nB Y\nB X\nA Y\nA Z\nC X\nB X\nC Y\nA X\nB X\nC Z\nC Z\nA X\nB X\nB X\nB Y\nB X\nB X\nA X\nA Z\nB Z\nB X\nA X\nB X\nB X\nB X\nA Z\nA X\nA X\nA Z\nB Y\nB Z\nA X\nA Z\nA Z\nA Z\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nA Z\nB Z\nA X\nA Z\nC Y\nA Z\nA X\nA Y\nA Y\nB X\nB Y\nA Z\nA X\nB X\nB Y\nA X\nB X\nA X\nB X\nB X\nC Y\nA Z\nB X\nA X\nB X\nA X\nA Z\nA X\nA Z\nA X\nA X\nA X\nB X\nA Y\nA X\nA X\nB X\nC Z\nB X\nA X\nC Y\nA X\nB X\nA Z\nB Y\nB Z\nA X\nA X\nA X\nC Y\nA X\nA X\nA X\nB X\nC Z\nA Z\nA X\nC Z\nB X\nA Z\nA Y\nC Y\nC Y\nA Z\nB X\nC Z\nC Z\nA X\nA X\nA X\nA X\nA X\nB X\nC Y\nA Z\nA X\nB X\nA Z\nA X\nA Y\nA X\nB Z\nA X\nB X\nA X\nA X\nA X\nA Y\nB X\nC Y\nA X\nA Z\nC Y\nA Z\nA X\nA Y\nA X\nA Z\nA X\nA Y\nC Y\nA Y\nA Z\nA X\nA X\nA X\nA Z\nA X\nB X\nA Y\nA X\nB X\nA X\nA X\nB X\nA X\nB X\nC Z\nB X\nA Z\nA Z\nC Y\nB Y\nB X\nA Y\nB X\nB X\nA Y\nA X\nC X\nC Y\nB X\nA X\nC X\nC Z\nA X\nA Z\nB X\nB Z\nC Y\nA X\nC Y\nA Y\nB X\nB Y\nA X\nC Y\nA Z\nA X\nA X\nC Y\nC Y\nA X\nA Z\nB X\nA Y\nB X\nA Z\nA Z\nA Z\nA Z\nA X\nC Z\nB X\nB Y\nA X\nA Y\nA X\nB X\nC Y\nC Z\nB Z\nB X\nB Z\nA X\nA Z\nC Y\nA X\nB Y\nA X\nA Z\nA X\nC Z\nA X\nA Y\nB Y\nA X\nA X\nA X\nA Z\nB X\nB X\nB X\nA X\nC Y\nC X\nC Y\nC Z\nA X\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA X\nA X\nA X\nC Z\nA X\nB X\nA Z\nB X\nB X\nB X\nA Y\nA X\nA X\nB X\nA X\nB X\nA Z\nC Y\nB Y\nC Z\nB Y\nB X\nB X\nA X\nC Z\nA X\nB Y\nC Y\nB Y\nA Z\nA Y\nA X\nA X\nC Y\nB X\nB X\nA Z\nA X\nC Y\nC Y\nA Y\nA Z\nB X\nC Z\nA Z\nC Y\nA X\nA X\nA Z\nB X\nA X\nA X\nA Z\nB X\nC Z\nA X\nA X\nB X\nB Y\nA Z\nA X\nB X\nA X\nC Y\nB X\nB X\nB Z\nC Y\nA X\nB X\nA X\nA X\nA Y\nA Y\nA Z\nB X\nA Z\nB Y\nB X\nA X\nB Y\nA X\nA X\nA Z\nA X\nB X\nA Z\nA X\nA Z\nB X\nB X\nB X\nB Z\nC Y\nA Y\nA Y\nA Z\nA X\nB X\nB X\nA X\nA X\nB Y\nA X\nB X\nA Z\nB Z\nB X\nC Y\nA Z\nA X\nC Z\nA Y\nA Y\nB X\nA Y\nC Z\nA Z\nA X\nB X\nA Z\nC Y\nC X\nB X\nB Z\nA X\nA Y\nA X\nB X\nA X\nA X\nC Y\nB Z\nB Z\nB Y\nC Z\nC Y\nB X\nB Y\nC Y\nA Z\nB X\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nB X\nB X\nA X\nC Y\nA X\nA Y\nA Z\nB Y\nC Y\nA Z\nA X\nA X\nA Z\nA Y\nB X\nA Y\nA X\nA X\nC Z\nA X\nA X\nB X\nA X\nA Z\nA Z\nA X\nA X\nA X\nB X\nA X\nA X\nA X\nA Z\nA X\nB X\nB Z\nC Y\nA X\nA Z\nB X\nB X\nB X\nA X\nB X\nA X\nB X\nA Y\nB Y\nC Y\nA X\nC Z\nB Z\nA Z\nA X\nA X\nA Z\nA Y\nA Z\nA X\nA X\nB X\nA X\nA X\nA Z\nC Z\nB X\nB Y\nA Z\nB X\nA Z\nA X\nA X\nA X\nC Y\nC Y\nA Z\nA X\nA X\nB Y\nA Y\nA X\nB Z\nB X\nB Z\nB X\nB Y\nB X\nA Y\nA Z\nC Y\nB X\nA X\nA X\nB X\nB Z\nA X\nC X\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nA X\nC Z\nA Y\nB X\nB X\nB Z\nA X\nB X\nB Z\nB X\nA Z\nA X\nB X\nA X\nA Y\nB X\nA Z\nC Z\nB X\nC Z\nA X\nA X\nC Y\nA Z\nC Y\nA Z\nA X\nB X\nA X\nC Y\nA Z\nA X\nA X\nB X\nB X\nA Z\nB Y\nB X\nA X\nA Z\nA X\nB X\nA X\nA Z\nA Z\nA Y\nA Y\nA X\nC Z\nA Z\nA Z\nA X\nB X\nA Y\nB Z\nA X\nA Z\nA X\nC Z\nB X\nA Z\nC Z\nA X\nC X\nA Z\nA X\nA X\nB Z\nA X\nB X\nA X\nB X\nC Z\nA X\nB X\nA Z\nB X\nC Y\nB X\nA Z\nA X\nA Y\nC Z\nA Z\nB X\nA Z\nA Y\nA Z\nB X\nA X\nB X\nC Y\nB X\nA X\nC Y\nA X\nA X\nB Y\nA X\nB Y\nA X\nA X\nA X\nB X\nA X\nA X\nC Z\nA X\nB X\nA Y\nA Y\nB Z\nA Z\nA Y\nA X\nB X\nB Y\nA Z\nB Y\nA X\nA Z\nA X\nA Z\nB X\nA X\nA X\nB X\nA Y\nB X\nB X\nA X\nA Z\nB X\nA Y\nB X\nA Z\nB X\nB X\nB X\nB X\nA Y\nA Z\nB X\nB Y\nB X\nC Y\nC Y\nC Y\nB X\nB X\nA Y\nA X\nB X\nB X\nA X\nB X\nA X\nA Z\nA Z\nA Z\nB X\nA X\nA X\nA Y\nA X\nA X\nC Y\nB X\nC Y\nA X\nA Y\nC Z\nA X\nB X\nA Z\nA X\nA X\nA Z\nA X\nB Z\nA X\nA Z\nA X\nC Y\nA Y\nC X\nA X\nC Y\nB X\nB X\nA Z\nA X\nA X\nA X\nB X\nA X\nC Y\nC Y\nA Y\nA X\nC Z\nA X\nB X\nB Y\nA Z\nA X\nB X\nB X\nA X\nA Z\nC Y\nB X\nA Y\nA X\nA Z\nB X\nA X\nA Z\nB X\nA Y\nB Z\nA Z\nC Z\nC Y\nA Z\nA X\nA X\nA Z\nB X\nA X\nB X\nA X\nA X\nB X\nA Z\nA X\nC Z\nB X\nA Z\nB Y\nA Z\nA X\nA Z\nB X\nA X\nA X\nA Y\nB X\nC Z\nB X\nA X\nA X\nA Z\nB X\nC X\nB X\nB X\nA Z\nB Y\nB X\nA Z\nC Y\nA X\nA X\nB X\nB X\nA X\nB X\nA Z\nA X\nA X\nA Z\nA X\nA X\nB X\nB X\nA X\nA Z\nA Y\nA X\nA X\nC Y\nB X\nA X\nB X\nC Z\nA Z\nB X\nA X\nA X\nA X\nC Y\nA X\nA X\nA X\nA Z\nB Y\nB X\nA X\nA X\nA Z\nB X\nC Z\nB X\nA Z\nA Z\nA X\nA Y\nB Y\nB X\nA Z\nA Z\nA X\nA Y\nB X\nA X\nA Z\nB X\nA Y\nA Z\nA Z\nB X\nC Z\nA X\nA X\nB X\nB X\nA X\nA Y\nA Y\nC Z\nB X\nA X\nA Z\nC Y\nB X\nA Z\nA X\nB X\nB Y\nA X\nB X\nA Y\nB X\nA X\nA Z\nB X\nB X\nC Y\nB X\nB X\nB Y\nA X\nA X\nB X\nB X\nA X\nB X\nC Y\nA X\nA X\nA Z\nA Z\nB X\nA Y\nB X\nB Y\nA X\nA Z\nA X\nA Z\nA X\nB Y\nB X\nB X\nA X\nB X\nA X\nB X\nB X\nA X\nB X\nC X\nA Z\nA Z\nA X\nA X\nA Z\nA X\nB X\nA X\nA X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nA Z\nA X\nA X\nA Y\nA X\nB Z\nC Z\nA Z\nB X\nA X\nB X\nA X\nB X\nB X\nA Y\nB Y\nA X\nB X\nB Y\nC Y\nB Z\nB Y\nB Y\nA Z\nA X\nA Y\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA Y\nC Y\nA X\nA Y\nA Z\nA Z\nB X\nA X\nA X\nA Z\nB Y\nB X\nB X\nA X\nB Z\nB Z\nA Y\nB X\nA Y\nA X\nB X\nC Z\nA X\nA Y\nB X\nA X\nB Z\nC Y\nA Z\nA X\nA Z\nA X\nA X\nA X\nA X\nC Y\nB X\nA X\nA X\nC Y\nC Y\nA Z\nC Y\nA Z\nA Y\nC X\nB X\nC Y\nA Z\nA X\nB X\nB Z\nB X\nB X\nB X\nB X\nB X\nA X\nA Z\nB X\nA Y\nA X\nA Y\nB X\nA X\nA X\nA X\nB Z\nB Z\nB X\nB X\nA Y\nA X\nB Y\nB X\nB Y\nC Y\nA X\nA X\nC Y\nA X\nB X\nB X\nB X\nC Z\nB X\nA Z\nB X\nB X\nA X\nB Z\nA Z\nA Z\nA X\nA Z\nA Y\nA Z\nB X\nA X\nB X\nA X\nA X\nA X\nB X\nA X\nB X\nA X\nA X\nB X\nA Z\nA X\nA Z\nC Y\nA X\nB X\nA X\nA X\nB X\nB X\nC Z\nA X\nA X\nB X\nA X\nA Y\nA X\nA X\nA Z\nA X\nA X\nB X\nA X\nA X\nA X\nA Y\nC Z\nA Z\nA Z\nA X\nB X\nA Z\nA X\nA X\nA X\nC Y\nC Y\nA Z\nC Y\nB X\nA X\nB X\nB X\nA Z\nA Z\nA Y\nB X\nA Z\nA X\nA Z\nA X\nA X\nA X\nA Z\nA Z\nB X\nA X\nA X\nB X\nB X\nA Z\nB X\nA X\nA X\nB X\nB X\nB X\nA X\nA X\nA Z\nA X\nA X\nB Y\nB Y\nC Y\nA Y\nA Y\nB X\nB X\nC Y\nA X\nB X\nA X\nA X\nA Z\nA Y\nA X\nB X\nA X\nB X\nA X\nA Z\nA X\nC Y\nB X\nB X\nA X\nB X\nA X\nA X\nA Z\nC Y\nB X\nB X\nA X\nA X\nA Z\nA Z\nB Y\nA X\nB X\nA Y\nC Z\nB X\nA X\nA Z\nB Y\nA Z\nB X\nB X\nB X\nA X\nA Z\nA Z\nB Y\nA Y\nB X\nB Y\nA Y\nA Z\nA X\nA X\nB X\nB X\nA Y\nB X\nA X\nA X\nC Y\nB X\nA X\nC X\nC Z\nA X\nA X\nC Y\nC Z\nA Z\nA X\nC Y\nC Y\nC Y\nA X\nA Y\nA Z\nA X\nC Z\nA X\nC X\nB X\nB Y\nB Z\nA X\nA X\nA X\nA Z\nB X\nB X\nC X\nA X\nC Z\nC Y\nA X\nA X\nB X\nB X\nA X\nA X\nA X\nA Z\nA Z\nB X\nA Z\nB Y\nA Y\nA Y\nA Z\nB X\nA Y\nA X\nA X\nB X\nA Z\nB X\nA Y\nA X\nB X\nA X\nB X\nA X\nB X\nB Z\nA Z\nA Y\nA Z\nA X\nB X\nB X\nC Y\nB X\nA X\nB X\nA Y\nB X\nB X\nC Y\nB X\nA Z\nA Z\nC Y\nA X\nB X\nB X\nA X\nB X\nA Z\nA X\nA X\nA Z\nA Z\nC Z\nA X\nA X\nA X\nA X\nA X\nA X\nA Z\nC Y\nA X\nC Y\nA X\nB X\nC Y\nB X\nA Y\nB X\nB X\nB X\nA X\nA X\nA X\nC Y\nA Z\nA Z\nB X\nA X\nA Z\nB X\nB X\nA X\nA X\nB Z\nA X\nB Z\nB X\nA X\nA X\nA X\nB X\nA X\nC Y\nA Y\nA X\nA X\nB X\nA Y\nA X\nC Y\nC Y\nB X\nB X\nA X\nA X\nC Y\nB Y\nC Y\nA X\nB X\nA Y\nB Z\nB X\nB X\nA Y\nB X\nA Z\nC Y\nB X\nB X\nB Y\nA X\nA X\nA X\nC Y\nB X\nA Y\nB X\nA Z\nB Y\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nC Z\nB X\nB X\nA X\nA X\nC Y\nA X\nA X\nA X\nA X\nB Z\nB X\nA Z\nB Z\nC Z\nC Y\nB X\nB X\nC Z\nA Z\nA Y\nA Z\nA Y\nC Z\nA X\nA X\nA Z\nA X\nA Z\nA X\nB Z\nB X\nB X\nC Z\nB X\nC Y\nC Y\nB X\nA X\nA X\nA X\nB Y\nA X\nC Y\nB Y\nC Y\nB Y\nA Z\nB X\nA X\nA Z\nB X\nB Z\nA Z\nA X\nA X\nA Y\nA X\nA X\nA Z\nB X\nB X\nB X\nB X\nB X\nA X\nA X\nA X\nA Z\nA Z\nA Y\nA X\nA Z\nA Z\nB X\nA X\nB X\nB X\nA Z\nB Y\nA X\nA X\nA X\nA X\nB X\nA Y\nA X\nC X\nA Z\nA X\nC Z\nC Y\nB X\nA Z\nA Z\nB X\nC Y\nA X\nA Z\nA X\nB Y\nC Y\nA X\nA X\nA Z\nA X\nC Y\nA Z\nA X\nA X\nA X\nC Y\nB X\nA X\nB Y\nA Z\nA X\nA Z\nB X\nA Z\nA Z\nA X\nA X\nC Z\nB X\nA Z\nA X\nB Z\nA Z\nA Z\nB X\nA X\nA Z\nA Y\nA X\nA Z\nA Z\nC X\nC Z\nB X\nB X\nA Z\nA X\nB X\nA X\nA X\nA X\nA Z\nB X\nA Z\nA X\nB X\nA X\nA X\nC Z\nB X\nC Y\nA X\nB X\nA Z\nA Z\nA X\nB Z\nB X\nA Z\nA Z\nA X\nC Y\nA X\nB X\nC Y\nB X\nB X\nA Z\nB X\nA Z\nC Y\nB X\nA Y\nB X\nA Z\nB Z\nA Z\nB X\nA X\nB Z\nA X\nA Y\nB Z\nC Y\nC Y\nB X\nA X\nA X\nA X\nA X\nB Z\nA X\nA X\nA X\nC Y\nC X\nB Y\nB X\nB X\nB X\nA Y\nA X\nA Z\nA X\nB X\nA Z\nC Y\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA Z\nA X\nA X\nB X\nC Z\nB X\nC Z\nB Y\nB X\nA Z\nA Z\nA Z\nB Y\nA Y\nA Z\nB X\nC Y\nA X\nB X\nA Z\nC Y\nA Y\nB X\nA X\nA Y\nA X\nA Z\nB X\nA Z\nB Y\nB X\nA X\nB X\nA X\nA X\nA X\nA Y\nB X\nA Y\nA X\nB X\nA X\nA X\nC Y\nA X\nA X\nB Z\nA Z\nA Z\nA X\nC Y\nB X\nB X\nC Y\nA Z\nC Y\nA X\nA X\nB X\nB X\nA X\nA Z\nB Y\nB X\nA Z\nB X\nA X\nB X\nA Z\nC Y\nA X\nB X\nA X\nA X\nA Z\nA X\nB X\nC Z\nA X\nA X\nA Y\nB X\nA Z\nB X\nB X\nB Y\nA X\nA Z\nA X\nA Z\nB X\nB X\nA X\nA X\nB Z\nA X\nC Y\nB X\nA X\nA Z\nA Z\nB X\nC Y\nC Y\nB Y\nA Z\nC Z\nA X\nB X\nA X\nA X\nB Y\nB X\nA X\nA X\nA Y\nA Z\nB X\nA X\nA X\nA Z\nA X\nA X\nB X\nB X\nA Y\nA X\nB X\nA Z\nB Z\nB X\nA Z\nA X\nA Z\nB X\nC Y\nA X\nA X\nB X\nA Z\nB X\nA Z\nC Z\nA Z\nA X\nC Y\nA X\nB X\nB Z\nA X\nB X\nC X\nA X\nB Y\nA X\nA X\nC Y\nA X\nB Y\nA X\nA X\nA X\nB X\nA X\nB X\nA Z\nC Y\nA X\nC Z\nA X\nB X\nA Z\nA X\nC Z\nC Z\nA X\nA Y\nA Z\nA X\nB X\nB X\nA Z\nB X\nA X\nB X\nB Y\nB X\nB X\nC Z\nA Y\nA X\nA Z\nB Z\nA X\nA Z\nA Z\nA Y\nB X\nA Y\nA X\nA X\nB X\nA Z\nB Z\nA X\nA Z\nA Z\nA Z\nC Y\nA X\nB Z\nB X\nA Z\nB X\nA Z\nA Y\nA X\nC Z\nC X\nA Y\nA X\nA Z\nA X\nA X\nA Z\nA Y\nA Z\nA Y\nA X\nC Y\nB X\nA Z\nA X\nA X\nC Y\nA X\nA X\nB Z\nA Z\nC Y\nA X\nA X\nA X\nB X\nC Y\nA X\nA Y\nB X\nA X\nA X\nB Y\nA Z\nA X\nA X\nB X\nA Z\nA Y\nC Y\nC Z\nC Y\nA X\nC Y\nC Y\nA X\nA Y\nC Y\nB X\nA X\nB X\nA X\nA X\nB X\nA X\nA Z\nC Y\nC Y\nB X\nA X\nB X\nC Z\nC Y\nB X\nA X\nA X\nA Y\nA X\nA Y\nA Z\nB X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nB X\nA Z\nA X\nA X\nA X\nB X\nB Z\nB X\nB X\nB X\nA Z\nA X\nB Z\nA X\nB Z\nA X\nB Y\nC Z\nB Y\nC Z\nB Y\nB X\nA X\nA X\nC Y\nB X\nB X\nA X\nB Z\nA Z\nA Z\nA X\nA X\nA X\nB X\nB X\nB X\nC Y\nA X\nB X\nB X\nB Z\nB X\nA Z\nC Y\nA X\nC Z\nA Z"));
}

/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/
