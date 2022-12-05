// https://adventofcode.com/2022/day/2
// (part 2)

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
const RESULTS_COUNT: usize = 3;

/*
                |           opponent
                | Rock     Paper     Scissors
    -----------------------------------------
            Loss| Scissors Rock     Paper
    result  Draw| Rock     Paper    Scissors
            Win | Paper    Scissors Rock
*/
#[rustfmt::skip]
const SHAPES_TO_DO: [Shape; SHAPES_COUNT * RESULTS_COUNT] = [
    Shape::Scissors, Shape::Rock,     Shape::Paper,
    Shape::Rock,     Shape::Paper,    Shape::Scissors,
    Shape::Paper,    Shape::Scissors, Shape::Rock,
];

fn shape_get_opponent(c: char) -> Shape {
    return match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("unexpected opponent shape [{}]", c),
    };
}

fn result_get(c: char) -> Result {
    return match c {
        'X' => Result::Loss,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => panic!("unexpected result [{}]", c),
    };
}

fn shape_compute_me(result: &Result, opponent: &Shape) -> Shape {
    let opponent_offset: usize = *opponent as usize;
    let result_offset: usize = (*result as usize) * RESULTS_COUNT;
    return SHAPES_TO_DO[opponent_offset + result_offset];
}

fn round_get(round: &str) -> (Shape, Result) {
    assert_eq!(round.len(), 3);
    assert_eq!(round.chars().nth(1).unwrap(), ' ');
    let opponent: Shape = shape_get_opponent(round.chars().nth(0).unwrap());
    let result: Result = result_get(round.chars().nth(2).unwrap());
    let me: Shape = shape_compute_me(&result, &opponent);
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
    assert_eq!(12, compute_score("A Y\nB X\nC Z"));
    //// user puzzle input
    assert_eq!(10238, compute_score("A X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nB X\nA Z\nA X\nA X\nA X\nA X\nB X\nA X\nB X\nA Z\nA Z\nA X\nA X\nA Z\nA Z\nA Z\nA X\nA Z\nA X\nC Y\nC Z\nA X\nB X\nA X\nA X\nA X\nA Z\nB X\nA X\nB X\nC Y\nA X\nA Z\nA Y\nA X\nA Z\nA X\nA Z\nB X\nA X\nA Z\nB X\nA X\nA Z\nA X\nA Z\nA Z\nA X\nB X\nB X\nB X\nB Z\nA Z\nB Z\nA X\nA X\nA Z\nA X\nB Z\nB Y\nA Z\nB X\nA Y\nB X\nC X\nB Z\nB X\nC Y\nA Z\nA X\nA X\nC Y\nB X\nA X\nA Y\nA X\nA X\nA Y\nA Z\nC Z\nA Z\nB X\nB X\nA X\nB X\nA X\nA X\nB Y\nB X\nA Z\nB X\nB X\nA X\nB Z\nC Z\nA Z\nC Z\nB X\nB X\nA Z\nA X\nA X\nB X\nA Z\nB X\nA Z\nA X\nA X\nA X\nB X\nB X\nB X\nB X\nA X\nA Y\nA X\nB X\nA Z\nA X\nA X\nA X\nC Y\nA X\nB X\nA X\nA Z\nA X\nB X\nA Y\nB X\nC Z\nA Y\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nB X\nA X\nA X\nA Z\nA Z\nA X\nC Y\nB X\nB X\nA Z\nB X\nB Y\nC Y\nA X\nA X\nB Y\nA X\nB Y\nB X\nB X\nA X\nA Z\nB X\nB X\nA Y\nA X\nC Y\nC Y\nC Y\nA X\nB X\nA X\nA X\nA X\nB X\nB X\nC Y\nA X\nA Z\nA Z\nA X\nB X\nA Z\nA Z\nA X\nB X\nA X\nA Z\nA X\nC Z\nB X\nA X\nC Y\nC Y\nA Z\nB X\nB Z\nC Y\nB X\nA Y\nB Y\nA Z\nA X\nA X\nB X\nB Y\nA Z\nA X\nB X\nA X\nC X\nC Y\nA X\nB X\nA Z\nA X\nB X\nA X\nC Y\nC Y\nC Y\nA X\nA X\nA Z\nA X\nC Y\nA Z\nB Y\nA X\nC Y\nB Z\nB X\nB X\nA Z\nB X\nA Y\nC X\nA X\nB X\nA Z\nC Y\nA X\nA Z\nC Z\nA X\nC Y\nB X\nC Y\nC Z\nA Y\nA Z\nC Y\nA X\nA X\nA X\nA Y\nB X\nB X\nC Y\nC Y\nA X\nA Z\nC Y\nB Z\nB X\nC Z\nB X\nB X\nC Z\nA X\nA X\nB X\nB X\nA X\nA X\nB X\nB X\nB X\nC Y\nA X\nA X\nA X\nA X\nB X\nA X\nA X\nB X\nB Z\nA X\nB X\nA X\nC Y\nB Y\nA X\nB X\nA Y\nB Z\nB X\nA X\nB Z\nB X\nA Y\nA X\nB X\nA X\nA Z\nB X\nA X\nA X\nC Z\nA X\nB X\nA X\nB Y\nA Z\nA X\nB X\nA X\nA Z\nB X\nB X\nB Y\nB X\nA X\nC Y\nA X\nA X\nB X\nB Y\nA X\nA X\nA X\nC Z\nC Z\nB Y\nB X\nB Z\nA X\nA Y\nA X\nC Y\nB X\nA X\nA X\nA X\nA Z\nC Y\nA X\nA X\nC Y\nA X\nA X\nA X\nA Z\nA Y\nA X\nA Y\nC Z\nA Z\nA X\nB X\nA Z\nC Z\nC Y\nA Z\nA Z\nC Y\nC Z\nC Z\nA X\nB X\nB X\nB Z\nC Y\nA X\nA X\nA X\nA Z\nA X\nB X\nB X\nC X\nA X\nA X\nB X\nA Y\nA X\nA X\nB Y\nC Z\nA X\nA X\nA Y\nB X\nB X\nB X\nC Z\nA Z\nA Y\nB Y\nA X\nB X\nB X\nB X\nB Z\nA Y\nB Y\nA Z\nA Z\nC Z\nB X\nA X\nA Y\nA Y\nC Y\nB Y\nA X\nA X\nA X\nA X\nB Y\nA X\nA Y\nA X\nA Z\nB X\nB X\nA Y\nA X\nA Y\nA Y\nA Z\nB X\nB X\nA Z\nB X\nA X\nA Z\nA Z\nB X\nA Z\nA X\nA Y\nA Z\nA X\nA Z\nB Y\nA X\nC Y\nB X\nC X\nC Y\nB Z\nC Y\nC Y\nA X\nA X\nA Y\nB X\nC Z\nB X\nC Y\nA X\nA Z\nC Y\nA X\nA Z\nA Z\nB Y\nA X\nA X\nB X\nB Y\nA X\nA X\nB X\nA X\nB X\nA X\nB X\nA X\nB Y\nA Z\nB X\nB X\nB X\nB X\nA Y\nA X\nA Y\nA Y\nC Z\nB Z\nA X\nB Y\nA X\nC Z\nB X\nA X\nA X\nA X\nA X\nA X\nC Z\nC Z\nC Z\nA X\nC Y\nA X\nC X\nC Z\nA X\nB X\nA Z\nA Z\nA Z\nA X\nA Z\nA Z\nB Z\nA X\nB X\nA Z\nC Y\nA X\nA Z\nA Z\nA Z\nB Z\nB X\nA X\nB X\nB X\nB X\nB X\nB Y\nB X\nA X\nA X\nA Z\nA Z\nA Z\nB Y\nA X\nA X\nA Y\nB X\nB X\nA Z\nA X\nB X\nA X\nA X\nA Z\nC Y\nA X\nB X\nA X\nB X\nA X\nB X\nA Z\nB X\nA X\nA Z\nA Z\nB X\nA Y\nA X\nA Z\nC Y\nA X\nA X\nA X\nB Y\nB X\nA Y\nA Z\nC X\nB X\nC Y\nA X\nB X\nC Z\nC Z\nA X\nB X\nB X\nB Y\nB X\nB X\nA X\nA Z\nB Z\nB X\nA X\nB X\nB X\nB X\nA Z\nA X\nA X\nA Z\nB Y\nB Z\nA X\nA Z\nA Z\nA Z\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nA Z\nB Z\nA X\nA Z\nC Y\nA Z\nA X\nA Y\nA Y\nB X\nB Y\nA Z\nA X\nB X\nB Y\nA X\nB X\nA X\nB X\nB X\nC Y\nA Z\nB X\nA X\nB X\nA X\nA Z\nA X\nA Z\nA X\nA X\nA X\nB X\nA Y\nA X\nA X\nB X\nC Z\nB X\nA X\nC Y\nA X\nB X\nA Z\nB Y\nB Z\nA X\nA X\nA X\nC Y\nA X\nA X\nA X\nB X\nC Z\nA Z\nA X\nC Z\nB X\nA Z\nA Y\nC Y\nC Y\nA Z\nB X\nC Z\nC Z\nA X\nA X\nA X\nA X\nA X\nB X\nC Y\nA Z\nA X\nB X\nA Z\nA X\nA Y\nA X\nB Z\nA X\nB X\nA X\nA X\nA X\nA Y\nB X\nC Y\nA X\nA Z\nC Y\nA Z\nA X\nA Y\nA X\nA Z\nA X\nA Y\nC Y\nA Y\nA Z\nA X\nA X\nA X\nA Z\nA X\nB X\nA Y\nA X\nB X\nA X\nA X\nB X\nA X\nB X\nC Z\nB X\nA Z\nA Z\nC Y\nB Y\nB X\nA Y\nB X\nB X\nA Y\nA X\nC X\nC Y\nB X\nA X\nC X\nC Z\nA X\nA Z\nB X\nB Z\nC Y\nA X\nC Y\nA Y\nB X\nB Y\nA X\nC Y\nA Z\nA X\nA X\nC Y\nC Y\nA X\nA Z\nB X\nA Y\nB X\nA Z\nA Z\nA Z\nA Z\nA X\nC Z\nB X\nB Y\nA X\nA Y\nA X\nB X\nC Y\nC Z\nB Z\nB X\nB Z\nA X\nA Z\nC Y\nA X\nB Y\nA X\nA Z\nA X\nC Z\nA X\nA Y\nB Y\nA X\nA X\nA X\nA Z\nB X\nB X\nB X\nA X\nC Y\nC X\nC Y\nC Z\nA X\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA X\nA X\nA X\nC Z\nA X\nB X\nA Z\nB X\nB X\nB X\nA Y\nA X\nA X\nB X\nA X\nB X\nA Z\nC Y\nB Y\nC Z\nB Y\nB X\nB X\nA X\nC Z\nA X\nB Y\nC Y\nB Y\nA Z\nA Y\nA X\nA X\nC Y\nB X\nB X\nA Z\nA X\nC Y\nC Y\nA Y\nA Z\nB X\nC Z\nA Z\nC Y\nA X\nA X\nA Z\nB X\nA X\nA X\nA Z\nB X\nC Z\nA X\nA X\nB X\nB Y\nA Z\nA X\nB X\nA X\nC Y\nB X\nB X\nB Z\nC Y\nA X\nB X\nA X\nA X\nA Y\nA Y\nA Z\nB X\nA Z\nB Y\nB X\nA X\nB Y\nA X\nA X\nA Z\nA X\nB X\nA Z\nA X\nA Z\nB X\nB X\nB X\nB Z\nC Y\nA Y\nA Y\nA Z\nA X\nB X\nB X\nA X\nA X\nB Y\nA X\nB X\nA Z\nB Z\nB X\nC Y\nA Z\nA X\nC Z\nA Y\nA Y\nB X\nA Y\nC Z\nA Z\nA X\nB X\nA Z\nC Y\nC X\nB X\nB Z\nA X\nA Y\nA X\nB X\nA X\nA X\nC Y\nB Z\nB Z\nB Y\nC Z\nC Y\nB X\nB Y\nC Y\nA Z\nB X\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nB X\nB X\nA X\nC Y\nA X\nA Y\nA Z\nB Y\nC Y\nA Z\nA X\nA X\nA Z\nA Y\nB X\nA Y\nA X\nA X\nC Z\nA X\nA X\nB X\nA X\nA Z\nA Z\nA X\nA X\nA X\nB X\nA X\nA X\nA X\nA Z\nA X\nB X\nB Z\nC Y\nA X\nA Z\nB X\nB X\nB X\nA X\nB X\nA X\nB X\nA Y\nB Y\nC Y\nA X\nC Z\nB Z\nA Z\nA X\nA X\nA Z\nA Y\nA Z\nA X\nA X\nB X\nA X\nA X\nA Z\nC Z\nB X\nB Y\nA Z\nB X\nA Z\nA X\nA X\nA X\nC Y\nC Y\nA Z\nA X\nA X\nB Y\nA Y\nA X\nB Z\nB X\nB Z\nB X\nB Y\nB X\nA Y\nA Z\nC Y\nB X\nA X\nA X\nB X\nB Z\nA X\nC X\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nA X\nC Z\nA Y\nB X\nB X\nB Z\nA X\nB X\nB Z\nB X\nA Z\nA X\nB X\nA X\nA Y\nB X\nA Z\nC Z\nB X\nC Z\nA X\nA X\nC Y\nA Z\nC Y\nA Z\nA X\nB X\nA X\nC Y\nA Z\nA X\nA X\nB X\nB X\nA Z\nB Y\nB X\nA X\nA Z\nA X\nB X\nA X\nA Z\nA Z\nA Y\nA Y\nA X\nC Z\nA Z\nA Z\nA X\nB X\nA Y\nB Z\nA X\nA Z\nA X\nC Z\nB X\nA Z\nC Z\nA X\nC X\nA Z\nA X\nA X\nB Z\nA X\nB X\nA X\nB X\nC Z\nA X\nB X\nA Z\nB X\nC Y\nB X\nA Z\nA X\nA Y\nC Z\nA Z\nB X\nA Z\nA Y\nA Z\nB X\nA X\nB X\nC Y\nB X\nA X\nC Y\nA X\nA X\nB Y\nA X\nB Y\nA X\nA X\nA X\nB X\nA X\nA X\nC Z\nA X\nB X\nA Y\nA Y\nB Z\nA Z\nA Y\nA X\nB X\nB Y\nA Z\nB Y\nA X\nA Z\nA X\nA Z\nB X\nA X\nA X\nB X\nA Y\nB X\nB X\nA X\nA Z\nB X\nA Y\nB X\nA Z\nB X\nB X\nB X\nB X\nA Y\nA Z\nB X\nB Y\nB X\nC Y\nC Y\nC Y\nB X\nB X\nA Y\nA X\nB X\nB X\nA X\nB X\nA X\nA Z\nA Z\nA Z\nB X\nA X\nA X\nA Y\nA X\nA X\nC Y\nB X\nC Y\nA X\nA Y\nC Z\nA X\nB X\nA Z\nA X\nA X\nA Z\nA X\nB Z\nA X\nA Z\nA X\nC Y\nA Y\nC X\nA X\nC Y\nB X\nB X\nA Z\nA X\nA X\nA X\nB X\nA X\nC Y\nC Y\nA Y\nA X\nC Z\nA X\nB X\nB Y\nA Z\nA X\nB X\nB X\nA X\nA Z\nC Y\nB X\nA Y\nA X\nA Z\nB X\nA X\nA Z\nB X\nA Y\nB Z\nA Z\nC Z\nC Y\nA Z\nA X\nA X\nA Z\nB X\nA X\nB X\nA X\nA X\nB X\nA Z\nA X\nC Z\nB X\nA Z\nB Y\nA Z\nA X\nA Z\nB X\nA X\nA X\nA Y\nB X\nC Z\nB X\nA X\nA X\nA Z\nB X\nC X\nB X\nB X\nA Z\nB Y\nB X\nA Z\nC Y\nA X\nA X\nB X\nB X\nA X\nB X\nA Z\nA X\nA X\nA Z\nA X\nA X\nB X\nB X\nA X\nA Z\nA Y\nA X\nA X\nC Y\nB X\nA X\nB X\nC Z\nA Z\nB X\nA X\nA X\nA X\nC Y\nA X\nA X\nA X\nA Z\nB Y\nB X\nA X\nA X\nA Z\nB X\nC Z\nB X\nA Z\nA Z\nA X\nA Y\nB Y\nB X\nA Z\nA Z\nA X\nA Y\nB X\nA X\nA Z\nB X\nA Y\nA Z\nA Z\nB X\nC Z\nA X\nA X\nB X\nB X\nA X\nA Y\nA Y\nC Z\nB X\nA X\nA Z\nC Y\nB X\nA Z\nA X\nB X\nB Y\nA X\nB X\nA Y\nB X\nA X\nA Z\nB X\nB X\nC Y\nB X\nB X\nB Y\nA X\nA X\nB X\nB X\nA X\nB X\nC Y\nA X\nA X\nA Z\nA Z\nB X\nA Y\nB X\nB Y\nA X\nA Z\nA X\nA Z\nA X\nB Y\nB X\nB X\nA X\nB X\nA X\nB X\nB X\nA X\nB X\nC X\nA Z\nA Z\nA X\nA X\nA Z\nA X\nB X\nA X\nA X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nA Z\nA X\nA X\nA Y\nA X\nB Z\nC Z\nA Z\nB X\nA X\nB X\nA X\nB X\nB X\nA Y\nB Y\nA X\nB X\nB Y\nC Y\nB Z\nB Y\nB Y\nA Z\nA X\nA Y\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA Y\nC Y\nA X\nA Y\nA Z\nA Z\nB X\nA X\nA X\nA Z\nB Y\nB X\nB X\nA X\nB Z\nB Z\nA Y\nB X\nA Y\nA X\nB X\nC Z\nA X\nA Y\nB X\nA X\nB Z\nC Y\nA Z\nA X\nA Z\nA X\nA X\nA X\nA X\nC Y\nB X\nA X\nA X\nC Y\nC Y\nA Z\nC Y\nA Z\nA Y\nC X\nB X\nC Y\nA Z\nA X\nB X\nB Z\nB X\nB X\nB X\nB X\nB X\nA X\nA Z\nB X\nA Y\nA X\nA Y\nB X\nA X\nA X\nA X\nB Z\nB Z\nB X\nB X\nA Y\nA X\nB Y\nB X\nB Y\nC Y\nA X\nA X\nC Y\nA X\nB X\nB X\nB X\nC Z\nB X\nA Z\nB X\nB X\nA X\nB Z\nA Z\nA Z\nA X\nA Z\nA Y\nA Z\nB X\nA X\nB X\nA X\nA X\nA X\nB X\nA X\nB X\nA X\nA X\nB X\nA Z\nA X\nA Z\nC Y\nA X\nB X\nA X\nA X\nB X\nB X\nC Z\nA X\nA X\nB X\nA X\nA Y\nA X\nA X\nA Z\nA X\nA X\nB X\nA X\nA X\nA X\nA Y\nC Z\nA Z\nA Z\nA X\nB X\nA Z\nA X\nA X\nA X\nC Y\nC Y\nA Z\nC Y\nB X\nA X\nB X\nB X\nA Z\nA Z\nA Y\nB X\nA Z\nA X\nA Z\nA X\nA X\nA X\nA Z\nA Z\nB X\nA X\nA X\nB X\nB X\nA Z\nB X\nA X\nA X\nB X\nB X\nB X\nA X\nA X\nA Z\nA X\nA X\nB Y\nB Y\nC Y\nA Y\nA Y\nB X\nB X\nC Y\nA X\nB X\nA X\nA X\nA Z\nA Y\nA X\nB X\nA X\nB X\nA X\nA Z\nA X\nC Y\nB X\nB X\nA X\nB X\nA X\nA X\nA Z\nC Y\nB X\nB X\nA X\nA X\nA Z\nA Z\nB Y\nA X\nB X\nA Y\nC Z\nB X\nA X\nA Z\nB Y\nA Z\nB X\nB X\nB X\nA X\nA Z\nA Z\nB Y\nA Y\nB X\nB Y\nA Y\nA Z\nA X\nA X\nB X\nB X\nA Y\nB X\nA X\nA X\nC Y\nB X\nA X\nC X\nC Z\nA X\nA X\nC Y\nC Z\nA Z\nA X\nC Y\nC Y\nC Y\nA X\nA Y\nA Z\nA X\nC Z\nA X\nC X\nB X\nB Y\nB Z\nA X\nA X\nA X\nA Z\nB X\nB X\nC X\nA X\nC Z\nC Y\nA X\nA X\nB X\nB X\nA X\nA X\nA X\nA Z\nA Z\nB X\nA Z\nB Y\nA Y\nA Y\nA Z\nB X\nA Y\nA X\nA X\nB X\nA Z\nB X\nA Y\nA X\nB X\nA X\nB X\nA X\nB X\nB Z\nA Z\nA Y\nA Z\nA X\nB X\nB X\nC Y\nB X\nA X\nB X\nA Y\nB X\nB X\nC Y\nB X\nA Z\nA Z\nC Y\nA X\nB X\nB X\nA X\nB X\nA Z\nA X\nA X\nA Z\nA Z\nC Z\nA X\nA X\nA X\nA X\nA X\nA X\nA Z\nC Y\nA X\nC Y\nA X\nB X\nC Y\nB X\nA Y\nB X\nB X\nB X\nA X\nA X\nA X\nC Y\nA Z\nA Z\nB X\nA X\nA Z\nB X\nB X\nA X\nA X\nB Z\nA X\nB Z\nB X\nA X\nA X\nA X\nB X\nA X\nC Y\nA Y\nA X\nA X\nB X\nA Y\nA X\nC Y\nC Y\nB X\nB X\nA X\nA X\nC Y\nB Y\nC Y\nA X\nB X\nA Y\nB Z\nB X\nB X\nA Y\nB X\nA Z\nC Y\nB X\nB X\nB Y\nA X\nA X\nA X\nC Y\nB X\nA Y\nB X\nA Z\nB Y\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nC Z\nB X\nB X\nA X\nA X\nC Y\nA X\nA X\nA X\nA X\nB Z\nB X\nA Z\nB Z\nC Z\nC Y\nB X\nB X\nC Z\nA Z\nA Y\nA Z\nA Y\nC Z\nA X\nA X\nA Z\nA X\nA Z\nA X\nB Z\nB X\nB X\nC Z\nB X\nC Y\nC Y\nB X\nA X\nA X\nA X\nB Y\nA X\nC Y\nB Y\nC Y\nB Y\nA Z\nB X\nA X\nA Z\nB X\nB Z\nA Z\nA X\nA X\nA Y\nA X\nA X\nA Z\nB X\nB X\nB X\nB X\nB X\nA X\nA X\nA X\nA Z\nA Z\nA Y\nA X\nA Z\nA Z\nB X\nA X\nB X\nB X\nA Z\nB Y\nA X\nA X\nA X\nA X\nB X\nA Y\nA X\nC X\nA Z\nA X\nC Z\nC Y\nB X\nA Z\nA Z\nB X\nC Y\nA X\nA Z\nA X\nB Y\nC Y\nA X\nA X\nA Z\nA X\nC Y\nA Z\nA X\nA X\nA X\nC Y\nB X\nA X\nB Y\nA Z\nA X\nA Z\nB X\nA Z\nA Z\nA X\nA X\nC Z\nB X\nA Z\nA X\nB Z\nA Z\nA Z\nB X\nA X\nA Z\nA Y\nA X\nA Z\nA Z\nC X\nC Z\nB X\nB X\nA Z\nA X\nB X\nA X\nA X\nA X\nA Z\nB X\nA Z\nA X\nB X\nA X\nA X\nC Z\nB X\nC Y\nA X\nB X\nA Z\nA Z\nA X\nB Z\nB X\nA Z\nA Z\nA X\nC Y\nA X\nB X\nC Y\nB X\nB X\nA Z\nB X\nA Z\nC Y\nB X\nA Y\nB X\nA Z\nB Z\nA Z\nB X\nA X\nB Z\nA X\nA Y\nB Z\nC Y\nC Y\nB X\nA X\nA X\nA X\nA X\nB Z\nA X\nA X\nA X\nC Y\nC X\nB Y\nB X\nB X\nB X\nA Y\nA X\nA Z\nA X\nB X\nA Z\nC Y\nA X\nA Z\nA Z\nA X\nA X\nA X\nA X\nA Z\nA X\nA X\nB X\nC Z\nB X\nC Z\nB Y\nB X\nA Z\nA Z\nA Z\nB Y\nA Y\nA Z\nB X\nC Y\nA X\nB X\nA Z\nC Y\nA Y\nB X\nA X\nA Y\nA X\nA Z\nB X\nA Z\nB Y\nB X\nA X\nB X\nA X\nA X\nA X\nA Y\nB X\nA Y\nA X\nB X\nA X\nA X\nC Y\nA X\nA X\nB Z\nA Z\nA Z\nA X\nC Y\nB X\nB X\nC Y\nA Z\nC Y\nA X\nA X\nB X\nB X\nA X\nA Z\nB Y\nB X\nA Z\nB X\nA X\nB X\nA Z\nC Y\nA X\nB X\nA X\nA X\nA Z\nA X\nB X\nC Z\nA X\nA X\nA Y\nB X\nA Z\nB X\nB X\nB Y\nA X\nA Z\nA X\nA Z\nB X\nB X\nA X\nA X\nB Z\nA X\nC Y\nB X\nA X\nA Z\nA Z\nB X\nC Y\nC Y\nB Y\nA Z\nC Z\nA X\nB X\nA X\nA X\nB Y\nB X\nA X\nA X\nA Y\nA Z\nB X\nA X\nA X\nA Z\nA X\nA X\nB X\nB X\nA Y\nA X\nB X\nA Z\nB Z\nB X\nA Z\nA X\nA Z\nB X\nC Y\nA X\nA X\nB X\nA Z\nB X\nA Z\nC Z\nA Z\nA X\nC Y\nA X\nB X\nB Z\nA X\nB X\nC X\nA X\nB Y\nA X\nA X\nC Y\nA X\nB Y\nA X\nA X\nA X\nB X\nA X\nB X\nA Z\nC Y\nA X\nC Z\nA X\nB X\nA Z\nA X\nC Z\nC Z\nA X\nA Y\nA Z\nA X\nB X\nB X\nA Z\nB X\nA X\nB X\nB Y\nB X\nB X\nC Z\nA Y\nA X\nA Z\nB Z\nA X\nA Z\nA Z\nA Y\nB X\nA Y\nA X\nA X\nB X\nA Z\nB Z\nA X\nA Z\nA Z\nA Z\nC Y\nA X\nB Z\nB X\nA Z\nB X\nA Z\nA Y\nA X\nC Z\nC X\nA Y\nA X\nA Z\nA X\nA X\nA Z\nA Y\nA Z\nA Y\nA X\nC Y\nB X\nA Z\nA X\nA X\nC Y\nA X\nA X\nB Z\nA Z\nC Y\nA X\nA X\nA X\nB X\nC Y\nA X\nA Y\nB X\nA X\nA X\nB Y\nA Z\nA X\nA X\nB X\nA Z\nA Y\nC Y\nC Z\nC Y\nA X\nC Y\nC Y\nA X\nA Y\nC Y\nB X\nA X\nB X\nA X\nA X\nB X\nA X\nA Z\nC Y\nC Y\nB X\nA X\nB X\nC Z\nC Y\nB X\nA X\nA X\nA Y\nA X\nA Y\nA Z\nB X\nA Z\nA Z\nA X\nA X\nB X\nC Y\nB X\nA Z\nA X\nA X\nA X\nB X\nB Z\nB X\nB X\nB X\nA Z\nA X\nB Z\nA X\nB Z\nA X\nB Y\nC Z\nB Y\nC Z\nB Y\nB X\nA X\nA X\nC Y\nB X\nB X\nA X\nB Z\nA Z\nA Z\nA X\nA X\nA X\nB X\nB X\nB X\nC Y\nA X\nB X\nB X\nB Z\nB X\nA Z\nC Y\nA X\nC Z\nA Z"));
}

/*
--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/
