// https://adventofcode.com/2022/day/9
// (part 2)

const KNOTS_COUNT: usize = 10;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn distance_compte(p1: &Point, p2: &Point) -> f64 {
    return f64::sqrt(((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64);
}

fn number_parse(number: &str) -> (usize, usize) {
    let mut digit_count: usize = 0;
    for c in number.chars() {
        if c < '0' || c > '9' {
            break;
        }
        digit_count += 1;
    }
    let n = number
        .get(0..digit_count)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    return (n, digit_count);
}

fn knot_follow(leader: Point, follower: &mut Point) {
    let distance = distance_compte(&leader, follower);
    if distance >= 2.0 {
        if leader.x > follower.x {
            follower.x += 1;
        } else if leader.x < follower.x {
            follower.x -= 1
        }
        if leader.y > follower.y {
            follower.y += 1;
        } else if leader.y < follower.y {
            follower.y -= 1
        }
    }
}

fn move_knots(
    knots: &mut [Point; KNOTS_COUNT],
    head_move: Point,
    count: usize,
    visited: &mut std::collections::HashSet<Point>,
) {
    for _ in 0..count {
        knots[0] += head_move;
        for i in 1..knots.len() {
            knot_follow(knots[i - 1], &mut knots[i]);
        }
        visited.insert(knots[knots.len() - 1]);
    }
}

fn count_visited(input: &str) -> usize {
    let mut visited: std::collections::HashSet<Point> = std::collections::HashSet::new();

    let mut knots: [Point; KNOTS_COUNT] = [Point { x: 0, y: 0 }; KNOTS_COUNT];
    visited.insert(knots[knots.len() - 1]);

    let commands: Vec<&str> = input.split('\n').collect();
    for command in commands {
        assert!(command.chars().nth(1).unwrap() == ' ');
        let direction: char = command.chars().nth(0).unwrap();
        let (move_count, chars_read_count) = number_parse(command.get(2..).unwrap());
        assert!(chars_read_count + 2 == command.len());
        match direction {
            'R' => move_knots(&mut knots, Point { x: 1, y: 0 }, move_count, &mut visited),
            'D' => move_knots(&mut knots, Point { x: 0, y: -1 }, move_count, &mut visited),
            'L' => move_knots(&mut knots, Point { x: -1, y: 0 }, move_count, &mut visited),
            'U' => move_knots(&mut knots, Point { x: 0, y: 1 }, move_count, &mut visited),
            _ => assert!(false),
        }
    }

    return visited.len();
}

fn main() {
    // example
    assert_eq!(1, count_visited("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"));
    assert_eq!(
        36,
        count_visited("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20")
    );

    assert_eq!(2369, count_visited("D 2\nU 1\nD 2\nR 2\nL 1\nU 2\nL 1\nU 2\nD 1\nR 1\nD 2\nR 1\nD 1\nU 2\nD 2\nR 1\nU 1\nR 1\nU 2\nR 1\nL 1\nU 2\nR 1\nL 2\nU 1\nR 1\nU 1\nR 1\nL 2\nU 2\nD 1\nU 1\nD 2\nL 1\nU 1\nL 1\nR 1\nD 1\nR 1\nD 1\nU 2\nD 1\nR 1\nU 1\nL 1\nU 2\nL 1\nD 1\nU 1\nD 1\nL 1\nU 2\nL 1\nD 2\nR 2\nL 1\nR 2\nD 1\nL 2\nD 2\nU 2\nR 1\nL 1\nD 2\nU 2\nL 1\nD 2\nU 1\nL 2\nR 2\nU 1\nD 1\nU 1\nD 1\nR 1\nD 1\nU 2\nD 1\nR 1\nD 1\nL 1\nD 1\nL 2\nU 2\nD 1\nR 1\nD 1\nL 1\nD 2\nR 2\nU 2\nD 1\nU 2\nD 1\nU 1\nL 1\nR 2\nD 2\nR 1\nD 2\nL 2\nD 2\nR 1\nL 1\nU 1\nD 2\nR 2\nU 2\nR 2\nL 1\nR 1\nL 3\nU 3\nL 3\nD 2\nU 1\nD 1\nU 2\nD 1\nR 1\nD 1\nR 2\nD 3\nU 1\nL 2\nU 1\nD 2\nR 1\nD 1\nL 2\nR 2\nU 1\nL 2\nD 2\nL 1\nU 2\nD 3\nL 1\nU 1\nR 2\nU 3\nD 1\nL 3\nU 1\nL 3\nD 1\nR 2\nL 3\nU 2\nL 2\nU 1\nL 1\nR 3\nD 3\nL 1\nR 2\nD 3\nR 2\nL 3\nD 1\nU 2\nR 3\nU 2\nR 3\nL 2\nU 1\nL 1\nD 3\nU 1\nD 1\nU 2\nD 3\nU 3\nR 1\nU 2\nL 1\nD 1\nL 2\nU 1\nD 1\nL 1\nR 3\nU 3\nL 3\nD 3\nL 2\nD 1\nU 1\nL 1\nU 3\nL 1\nD 3\nR 1\nL 1\nR 2\nU 1\nR 2\nU 3\nR 1\nD 3\nL 3\nU 2\nL 3\nR 1\nD 2\nU 3\nR 3\nD 3\nU 1\nR 3\nD 2\nR 3\nU 1\nD 1\nU 3\nR 1\nL 1\nU 2\nL 1\nU 1\nL 1\nR 1\nD 1\nR 3\nD 4\nL 2\nR 4\nU 3\nD 1\nR 4\nL 4\nR 1\nD 2\nU 3\nR 2\nD 2\nU 3\nR 4\nL 2\nD 3\nL 4\nD 4\nU 4\nR 2\nU 1\nL 3\nD 4\nU 2\nR 1\nL 3\nD 3\nR 4\nD 2\nR 2\nD 4\nU 4\nL 4\nD 4\nR 2\nD 2\nU 3\nD 3\nR 3\nL 2\nD 2\nL 2\nU 3\nL 4\nU 2\nR 4\nD 4\nL 3\nR 1\nU 1\nR 2\nL 1\nD 1\nU 2\nD 3\nL 4\nU 3\nL 1\nU 3\nR 2\nU 3\nR 1\nU 1\nR 2\nU 2\nR 1\nD 4\nU 4\nL 2\nU 2\nD 4\nU 1\nR 4\nL 4\nD 4\nU 3\nD 4\nL 4\nU 3\nL 1\nR 4\nL 3\nD 3\nU 1\nR 4\nD 3\nU 4\nL 2\nU 4\nD 3\nR 2\nD 3\nU 4\nR 4\nL 4\nR 3\nU 1\nR 4\nD 1\nL 3\nU 3\nL 3\nD 1\nL 4\nR 2\nU 1\nL 3\nR 2\nU 4\nL 1\nU 5\nR 2\nL 2\nU 2\nL 2\nR 4\nD 4\nL 5\nD 1\nR 4\nU 5\nL 1\nD 4\nU 1\nL 4\nU 3\nL 1\nR 3\nD 1\nR 2\nL 5\nU 3\nL 5\nU 4\nD 3\nL 5\nD 2\nR 4\nU 2\nL 1\nR 4\nL 5\nR 3\nL 1\nU 2\nL 4\nD 1\nR 2\nL 5\nU 3\nD 2\nR 5\nL 4\nD 2\nR 3\nD 2\nR 4\nD 3\nU 1\nD 1\nU 4\nD 2\nL 2\nR 1\nU 5\nD 5\nU 3\nR 2\nU 3\nL 5\nR 1\nU 3\nL 4\nR 2\nU 3\nD 1\nU 2\nD 2\nL 5\nR 5\nD 3\nL 1\nR 3\nU 2\nD 5\nU 1\nD 5\nR 3\nU 1\nL 5\nR 1\nD 4\nL 4\nD 1\nU 2\nL 4\nR 5\nU 5\nL 4\nD 2\nR 5\nU 1\nL 5\nD 1\nL 1\nU 5\nR 1\nL 2\nR 5\nL 5\nR 2\nL 1\nD 2\nU 1\nL 3\nR 1\nU 5\nD 1\nU 1\nL 5\nU 6\nR 6\nU 5\nD 5\nR 3\nU 1\nR 1\nL 6\nR 6\nD 5\nR 6\nD 4\nL 3\nR 6\nU 5\nD 6\nL 2\nR 1\nD 5\nR 3\nD 2\nL 1\nR 5\nU 5\nD 4\nU 3\nR 5\nU 5\nR 3\nD 5\nR 3\nU 3\nR 2\nD 5\nL 6\nD 3\nU 3\nD 6\nR 5\nU 4\nL 4\nD 3\nU 6\nL 2\nR 1\nU 4\nD 1\nL 2\nU 1\nL 3\nD 4\nU 1\nL 5\nD 1\nR 2\nU 3\nL 1\nR 1\nU 1\nL 4\nU 6\nR 6\nD 3\nU 4\nR 2\nD 5\nL 6\nD 2\nR 4\nD 2\nU 5\nD 3\nU 2\nR 6\nL 5\nR 6\nU 1\nL 5\nU 1\nL 6\nU 1\nD 4\nR 3\nD 3\nL 1\nD 3\nR 2\nL 1\nR 3\nD 4\nL 6\nU 2\nD 5\nU 3\nR 6\nU 2\nR 6\nL 5\nR 1\nU 2\nR 3\nU 1\nR 6\nL 2\nU 6\nR 5\nU 1\nR 4\nL 1\nR 3\nU 3\nL 5\nD 7\nR 2\nD 4\nR 5\nL 3\nR 6\nU 7\nL 7\nU 6\nL 2\nD 2\nR 7\nD 7\nU 3\nR 1\nU 7\nL 2\nD 3\nL 5\nR 4\nU 7\nR 3\nL 7\nD 2\nR 7\nD 5\nL 7\nD 1\nU 7\nR 5\nU 4\nD 4\nU 4\nD 4\nU 1\nR 7\nL 3\nD 3\nU 2\nL 3\nU 1\nD 7\nL 6\nR 3\nU 6\nD 1\nR 6\nD 1\nR 1\nD 7\nU 4\nR 1\nL 4\nU 3\nL 7\nR 2\nD 2\nL 6\nU 2\nR 2\nU 7\nR 5\nL 6\nR 5\nD 1\nR 4\nU 2\nL 1\nD 4\nU 5\nR 4\nD 7\nU 7\nR 1\nU 3\nR 4\nL 1\nR 6\nL 4\nU 4\nL 2\nU 3\nD 2\nL 2\nU 2\nL 6\nR 5\nD 7\nL 2\nU 5\nL 6\nR 7\nL 7\nD 2\nU 6\nD 3\nR 1\nU 1\nD 4\nL 7\nD 7\nU 5\nL 6\nD 1\nL 4\nD 1\nU 1\nL 5\nU 1\nR 5\nD 8\nR 7\nL 5\nU 3\nD 2\nR 2\nL 6\nR 1\nL 4\nR 8\nU 8\nL 5\nR 7\nD 7\nL 2\nR 5\nL 6\nD 2\nL 7\nU 3\nR 2\nD 3\nU 3\nL 6\nU 5\nD 3\nL 6\nU 5\nD 5\nL 1\nU 4\nL 8\nD 3\nL 3\nR 2\nU 6\nD 8\nU 7\nL 6\nD 2\nU 4\nD 1\nU 3\nL 6\nR 2\nD 2\nU 4\nR 8\nD 3\nR 6\nD 6\nU 1\nL 4\nU 3\nL 6\nD 8\nU 7\nD 5\nL 3\nU 4\nD 2\nR 7\nL 8\nU 1\nR 3\nD 5\nU 5\nD 6\nR 3\nL 2\nR 8\nL 3\nU 5\nL 3\nU 5\nL 7\nU 7\nL 2\nU 1\nL 5\nD 7\nU 7\nD 8\nR 2\nD 7\nU 4\nR 4\nU 6\nL 2\nU 8\nD 1\nU 1\nD 4\nL 4\nU 1\nL 5\nR 8\nU 5\nD 4\nR 2\nD 6\nL 1\nD 8\nU 4\nD 5\nL 8\nU 3\nL 5\nD 1\nU 3\nR 1\nD 7\nR 9\nU 8\nR 7\nL 3\nD 8\nU 9\nD 1\nU 7\nL 6\nU 4\nR 8\nU 1\nD 6\nR 3\nL 5\nD 3\nU 1\nL 3\nU 2\nL 3\nU 2\nR 7\nD 2\nR 7\nD 8\nR 1\nD 4\nR 1\nD 6\nR 8\nU 8\nL 2\nU 8\nL 5\nD 1\nU 7\nR 4\nL 9\nU 2\nL 3\nU 4\nL 2\nD 3\nL 4\nU 9\nD 2\nR 2\nU 9\nR 1\nL 4\nD 5\nR 9\nU 3\nL 3\nU 8\nR 5\nU 7\nD 5\nU 9\nR 3\nD 9\nR 1\nD 7\nR 3\nD 6\nL 6\nD 5\nU 9\nD 8\nU 4\nL 5\nR 6\nD 7\nR 6\nL 8\nR 3\nU 3\nR 9\nU 8\nR 6\nL 9\nU 2\nR 9\nL 8\nU 1\nR 9\nL 5\nD 5\nU 9\nL 6\nD 7\nU 3\nD 3\nR 4\nD 1\nL 4\nD 5\nL 4\nU 2\nR 9\nU 7\nL 4\nD 7\nU 9\nR 4\nD 7\nR 7\nD 4\nL 1\nD 2\nL 1\nU 7\nR 8\nD 2\nU 9\nD 6\nL 7\nR 5\nL 3\nR 5\nD 8\nL 3\nR 8\nL 6\nU 6\nL 6\nU 5\nR 7\nL 9\nU 1\nR 3\nD 7\nU 10\nD 1\nU 8\nD 8\nL 5\nU 6\nD 5\nU 9\nR 8\nD 8\nR 9\nD 2\nL 1\nD 8\nR 2\nD 6\nL 10\nR 1\nU 3\nL 8\nU 7\nR 7\nU 7\nD 6\nL 7\nU 3\nL 4\nR 9\nD 8\nU 4\nD 3\nL 8\nR 7\nU 1\nL 9\nD 10\nU 2\nL 5\nU 3\nL 4\nR 10\nL 2\nD 6\nL 6\nD 2\nU 5\nD 3\nR 9\nD 2\nU 1\nD 5\nU 9\nD 7\nR 2\nU 5\nR 4\nD 9\nU 6\nL 10\nD 3\nU 1\nL 1\nD 9\nU 2\nL 8\nU 3\nR 5\nU 7\nD 8\nR 5\nL 8\nU 9\nL 1\nD 5\nU 8\nR 4\nD 7\nU 4\nR 10\nU 9\nR 9\nU 7\nR 3\nU 10\nR 5\nU 7\nR 6\nU 10\nR 9\nU 8\nD 8\nL 1\nU 2\nD 1\nR 9\nU 10\nD 6\nL 3\nD 4\nR 11\nD 11\nR 7\nU 3\nL 3\nU 11\nD 11\nU 5\nR 9\nU 3\nR 4\nD 1\nU 11\nR 2\nL 7\nR 8\nL 2\nD 4\nU 4\nL 9\nR 10\nL 3\nR 7\nD 7\nL 9\nU 6\nL 5\nD 8\nU 4\nD 4\nL 4\nD 2\nU 8\nR 4\nD 4\nU 10\nD 5\nR 5\nD 5\nU 8\nD 2\nL 6\nU 8\nL 2\nD 8\nR 5\nD 5\nU 1\nD 1\nL 9\nR 7\nU 2\nL 7\nR 1\nL 1\nU 2\nR 10\nD 2\nU 8\nL 1\nR 6\nL 3\nR 4\nD 6\nL 2\nD 11\nR 8\nD 11\nL 3\nR 4\nL 10\nU 7\nL 9\nU 4\nL 8\nD 6\nU 8\nR 7\nL 5\nR 8\nL 4\nU 6\nR 5\nU 7\nD 3\nL 8\nU 2\nR 7\nD 3\nR 4\nD 11\nL 8\nU 6\nD 1\nR 11\nL 7\nD 4\nR 4\nL 4\nU 5\nL 8\nR 7\nU 8\nD 12\nR 9\nD 8\nR 7\nL 9\nR 2\nD 1\nL 7\nD 3\nL 1\nR 5\nU 1\nD 12\nR 4\nU 1\nD 4\nL 9\nU 10\nR 5\nD 11\nR 2\nL 3\nU 8\nR 5\nD 8\nU 12\nD 6\nR 2\nD 5\nR 12\nU 7\nL 2\nU 4\nD 5\nU 8\nR 12\nU 1\nL 11\nU 7\nD 10\nL 6\nD 6\nR 1\nL 6\nD 2\nU 5\nR 6\nD 8\nR 4\nL 7\nD 2\nL 7\nD 8\nU 10\nR 1\nU 6\nR 5\nL 4\nR 8\nD 9\nR 5\nU 11\nD 4\nU 11\nR 8\nL 8\nU 2\nR 11\nL 12\nU 7\nL 12\nU 10\nR 3\nD 11\nR 1\nL 8\nD 2\nL 5\nR 4\nU 3\nD 9\nR 8\nD 11\nL 12\nR 11\nL 4\nR 10\nD 6\nU 4\nR 6\nD 6\nU 9\nR 12\nU 5\nR 8\nL 10\nD 1\nU 7\nD 12\nL 3\nU 10\nD 2\nU 6\nD 12\nR 3\nU 2\nD 2\nU 3\nD 11\nR 2\nU 2\nD 9\nR 11\nD 4\nU 6\nR 11\nL 9\nR 10\nD 13\nL 2\nD 2\nL 4\nD 13\nR 7\nL 3\nU 8\nD 5\nR 7\nD 7\nU 13\nD 5\nU 5\nL 12\nD 9\nL 2\nD 6\nU 13\nR 2\nD 10\nL 9\nD 7\nR 2\nU 3\nD 2\nL 3\nR 9\nU 3\nL 12\nU 4\nR 1\nU 13\nR 4\nU 1\nD 10\nR 10\nD 10\nU 5\nL 7\nD 8\nR 1\nD 12\nL 13\nR 10\nD 6\nR 9\nU 4\nR 9\nU 1\nL 3\nR 3\nD 8\nU 6\nL 11\nR 8\nD 5\nU 2\nL 9\nR 7\nL 7\nD 8\nL 12\nR 12\nU 9\nD 9\nU 7\nD 10\nL 3\nR 3\nD 8\nL 11\nU 8\nD 7\nU 11\nL 4\nD 4\nR 3\nU 4\nR 6\nD 5\nU 5\nL 8\nU 4\nL 9\nU 6\nL 8\nD 3\nU 12\nD 5\nL 8\nD 11\nU 4\nR 12\nU 7\nL 5\nD 4\nR 4\nU 7\nR 9\nL 5\nU 2\nL 3\nD 2\nR 6\nU 2\nR 8\nD 10\nU 14\nL 3\nR 4\nU 10\nR 7\nL 11\nR 7\nU 4\nR 8\nL 14\nD 11\nU 12\nR 10\nL 8\nR 11\nL 4\nU 1\nL 2\nR 7\nU 13\nD 5\nU 2\nL 6\nD 7\nR 5\nU 4\nD 10\nU 5\nD 2\nR 9\nU 13\nL 10\nU 1\nR 10\nD 14\nL 5\nR 12\nL 7\nD 2\nL 2\nD 5\nR 4\nU 6\nD 7\nR 12\nU 9\nD 9\nR 6\nU 1\nD 14\nU 14\nD 14\nL 7\nR 10\nU 3\nD 2\nR 10\nL 14\nR 7\nD 5\nU 1\nL 8\nU 2\nD 3\nU 6\nL 11\nR 13\nL 10\nD 9\nU 5\nR 6\nU 2\nD 7\nU 5\nL 6\nR 8\nD 11\nR 14\nD 5\nR 10\nL 13\nR 1\nU 4\nL 9\nR 2\nD 1\nU 10\nD 6\nU 3\nR 13\nL 9\nR 2\nD 3\nR 14\nL 2\nR 14\nU 1\nR 7\nL 1\nU 6\nD 6\nR 10\nL 12\nD 10\nR 11\nD 13\nL 3\nR 2\nL 1\nR 10\nU 1\nL 7\nR 5\nD 6\nU 11\nL 8\nR 7\nD 10\nU 13\nR 12\nD 8\nL 7\nU 8\nR 14\nU 10\nL 6\nD 15\nR 8\nL 10\nU 13\nL 3\nU 13\nL 9\nR 4\nD 6\nR 12\nL 12\nR 4\nU 12\nD 6\nR 2\nL 11\nR 7\nD 1\nU 4\nR 14\nU 6\nD 1\nU 15\nD 14\nR 3\nD 14\nL 1\nR 9\nL 2\nD 11\nR 11\nD 10\nR 1\nL 9\nU 6\nR 8\nL 4\nR 9\nU 13\nD 15\nL 2\nD 2\nL 2\nD 6\nR 10\nU 10\nL 8\nD 11\nL 8\nR 10\nU 3\nR 7\nU 15\nR 11\nL 10\nU 2\nR 8\nL 8\nD 5\nR 11\nU 7\nL 13\nD 15\nU 4\nD 10\nR 11\nD 6\nL 2\nU 3\nR 2\nL 1\nD 14\nU 5\nL 14\nD 13\nR 12\nU 8\nR 1\nL 14\nU 6\nD 10\nR 3\nU 8\nL 2\nR 6\nD 10\nU 9\nL 2\nD 13\nL 13\nD 9\nL 10\nU 14\nR 11\nL 14\nU 10\nL 6\nD 14\nL 11\nR 4\nL 9\nU 3\nD 13\nL 5\nR 16\nU 3\nL 9\nD 6\nR 15\nU 6\nL 4\nR 10\nD 12\nR 4\nD 2\nU 11\nD 13\nR 6\nU 15\nR 14\nD 14\nU 8\nR 9\nL 5\nD 16\nU 9\nR 15\nD 13\nR 3\nL 2\nR 3\nU 14\nD 9\nU 2\nD 6\nU 9\nD 7\nL 13\nD 7\nL 5\nR 3\nU 2\nL 13\nR 9\nD 1\nL 16\nD 8\nR 12\nU 15\nR 4\nD 15\nL 12\nD 13\nR 4\nL 2\nU 5\nR 13\nL 15\nD 12\nL 3\nD 13\nL 5\nD 16\nL 11\nD 13\nU 1\nD 9\nL 10\nU 11\nD 12\nU 11\nD 14\nU 16\nD 14\nU 10\nR 7\nL 11\nR 14\nU 11\nR 5\nU 1\nD 16\nR 16\nL 9\nU 7\nD 7\nR 8\nD 7\nU 10\nL 7\nD 6\nU 4\nD 2\nL 3\nD 3\nL 2\nR 3\nL 9\nD 2\nR 12\nD 11\nU 11\nL 14\nU 4\nL 12\nR 13\nL 9\nD 11\nR 12\nL 15\nR 15\nL 15\nR 16\nU 17\nD 4\nL 9\nU 9\nD 17\nL 8\nU 15\nD 17\nU 9\nR 4\nU 6\nL 1\nD 7\nR 3\nD 11\nL 10\nR 1\nU 14\nD 13\nR 7\nL 8\nU 16\nL 16\nU 12\nD 3\nR 4\nD 8\nR 8\nD 1\nL 12\nR 16\nD 3\nR 7\nD 16\nU 11\nR 6\nD 13\nR 4\nL 3\nR 6\nU 1\nR 4\nD 4\nU 13\nL 7\nD 17\nR 4\nD 6\nL 1\nU 17\nL 16\nU 12\nL 11\nR 1\nL 11\nD 8\nU 15\nD 6\nL 5\nU 16\nR 6\nD 11\nL 17\nD 7\nL 1\nD 8\nL 16\nU 8\nR 4\nL 17\nD 2\nL 1\nU 16\nD 8\nL 17\nU 12\nD 4\nR 1\nD 12\nU 2\nL 11\nU 17\nL 3\nR 7\nL 10\nU 16\nR 8\nU 6\nL 3\nD 15\nL 11\nR 7\nU 14\nR 8\nL 9\nD 6\nL 3\nU 17\nR 1\nD 13\nU 15\nL 18\nU 11\nR 3\nU 16\nD 12\nR 1\nD 6\nL 17\nR 9\nL 6\nU 2\nD 2\nU 16\nD 12\nR 1\nU 17\nR 16\nD 12\nU 3\nD 11\nL 13\nU 11\nR 15\nU 13\nD 11\nU 8\nL 3\nD 1\nL 7\nR 9\nD 18\nR 17\nU 2\nL 16\nD 8\nL 14\nD 11\nR 4\nD 12\nU 5\nD 6\nU 2\nD 4\nL 1\nR 11\nD 3\nU 10\nL 1\nR 16\nU 10\nL 17\nD 5\nR 16\nU 18\nR 1\nD 16\nR 14\nD 11\nU 13\nD 13\nR 5\nL 9\nU 11\nR 18\nL 14\nU 14\nL 18\nR 15\nD 10\nU 10\nR 17\nU 12\nL 2\nD 11\nR 15\nU 3\nR 17\nL 1\nR 1\nU 1\nL 8\nU 15\nR 15\nL 12\nU 2\nR 1\nL 9\nR 3\nL 6\nR 18\nD 9\nR 12\nU 6\nL 17\nR 11\nD 8\nR 17\nD 12\nL 2\nR 9\nU 4\nD 17\nR 18\nD 12\nL 9\nR 16\nU 5\nR 1\nL 12\nR 9\nL 8\nU 8\nR 2\nU 9\nL 2\nD 18\nU 11\nL 17\nU 3\nL 11\nR 3\nL 5\nD 1\nR 4\nU 10\nD 18\nR 10\nD 10\nL 5\nR 7\nD 2\nR 2\nL 18\nU 4\nD 10\nU 16\nD 1\nL 2\nU 17\nD 5\nU 9\nR 13\nD 6\nR 14\nD 10\nL 15\nR 13\nL 6\nU 11\nR 6\nL 4\nR 19\nL 4\nD 13\nR 1\nL 9\nD 4\nR 12\nU 18\nD 3\nU 17\nR 19\nL 6\nR 6\nD 8\nR 12\nL 10\nR 3\nU 5\nL 2\nD 7\nL 9\nD 16\nL 16\nU 14\nL 12\nR 2\nL 17\nR 18\nD 12\nR 16\nD 19\nU 6\nD 19\nU 13\nR 4\nD 18\nL 14\nU 14\nD 12\nR 10\nL 13\nD 10\nU 10\nD 2\nU 18\nL 19\nR 15\nD 6\nL 1\nD 10\nR 7\nD 2\nR 19\nD 5\nU 19\nL 13\nU 17\nL 3\nR 19\nU 9\nL 7\nD 6\nR 12\nL 10\nR 9\nD 9\nU 13\nR 7\nL 1\nU 1\nD 17"));
}

/*
--- Part Two ---

A rope snaps! Suddenly, the river is getting a lot closer than you remember. The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!

The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit. Fortunately, your simulation can be extended to support longer ropes.

Rather than two knots, you now must simulate a rope consisting of ten knots. One knot is still the head of the rope and moves according to the series of motions. Each knot further down the rope follows the knot in front of it using the same rules as before.

Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now occur as follows:

== Initial State ==

......
......
......
......
H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 4 ==

......
......
......
......
1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
321H..  (3 covers 4, 5, 6, 7, 8, 9, s)

......
......
......
......
4321H.  (4 covers 5, 6, 7, 8, 9, s)

== U 4 ==

......
......
......
....H.
4321..  (4 covers 5, 6, 7, 8, 9, s)

......
......
....H.
.4321.
5.....  (5 covers 6, 7, 8, 9, s)

......
....H.
....1.
.432..
5.....  (5 covers 6, 7, 8, 9, s)

....H.
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

== L 3 ==

...H..
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

..H1..
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

.H1...
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

..1...
.H.2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== R 4 ==

..1...
..H2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

..1...
...H..  (H covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...1H.  (1 covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21H
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

......
...21.
..43.H
.5....
6.....  (6 covers 7, 8, 9, s)

== L 5 ==

......
...21.
..43H.
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21.
..4H..  (H covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
..H1..  (H covers 4; 1 covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
.H13..  (1 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
H123..  (2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

== R 2 ==

......
......
.H23..  (H covers 1; 2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
.1H3..  (H covers 2, 4)
.5....
6.....  (6 covers 7, 8, 9, s)

Now, you need to keep track of the positions the new tail, 9, visits. In this example, the tail never moves, and so it only visits 1 position. However, be careful: more types of motion are possible than before, so you might want to visually compare your simulated rope to the one above.

Here's a larger example:

R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

These motions occur as follows (individual steps are not shown):

== Initial State ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........H..............  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== R 5 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........54321H.........  (5 covers 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== U 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
................H.........
................1.........
................2.........
................3.........
...............54.........
..............6...........
.............7............
............8.............
...........9..............  (9 covers s)
..........................
..........................
..........................
..........................
..........................

== L 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
........H1234.............
............5.............
............6.............
............7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 3 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
.........2345.............
........1...6.............
........H...7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== R 17 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
................987654321H
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 10 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s.........98765
.........................4
.........................3
.........................2
.........................1
.........................H

== L 25 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
H123456789................

== U 20 ==

H.........................
1.........................
2.........................
3.........................
4.........................
5.........................
6.........................
7.........................
8.........................
9.........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

Now, the tail (9) visits 36 positions (including s) at least once:

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
#.........................
#.............###.........
#............#...#........
.#..........#.....#.......
..#..........#.....#......
...#........#.......#.....
....#......s.........#....
.....#..............#.....
......#............#......
.......#..........#.......
........#........#........
.........########.........

Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?
*/
