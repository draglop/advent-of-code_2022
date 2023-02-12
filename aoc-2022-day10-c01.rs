// https://adventofcode.com/2022/day/10
// (part 1)

const COMMAND_ADD: &str = "addx ";
const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const X_START_VALUE: i64 = 1;

fn number_parse(number: &str) -> (i64, usize) {
    let mut digit_count: usize = 0;
    let negative: bool = number.chars().nth(0).unwrap() == '-';
    let mut chars = number.chars();
    if negative {
        digit_count += 1;
        chars.next();
    }
    for c in chars {
        if c < '0' || c > '9' {
            break;
        }
        digit_count += 1;
    }
    let n = number.get(0..digit_count).unwrap().parse::<i64>().unwrap();

    return (n, digit_count);
}

fn signal_strengths(input: &str) -> i64 {
    let mut strengths: [i64; CYCLES.len()] = [0; CYCLES.len()];

    let commands: Vec<&str> = input.split('\n').collect();
    let mut x: i64 = X_START_VALUE;
    let mut strength_index: usize = 0;
    let mut cycles_count: usize = 0;
    for command in commands {
        let mut value: Option<i64> = Option::None;
        if command == "noop" {
            cycles_count += 1;
        } else if command.starts_with(COMMAND_ADD) {
            let (add_value, chars_read_count) =
                number_parse(command.get(COMMAND_ADD.len()..).unwrap());
            assert!(command.len() == chars_read_count + COMMAND_ADD.len());
            cycles_count += 2;
            value = Some(add_value);
        } else {
            assert!(false);
        }
        if cycles_count >= CYCLES[strength_index] {
            strengths[strength_index] = x * CYCLES[strength_index] as i64;
            strength_index += 1;
            if strength_index == CYCLES.len() {
                break;
            }
        }
        if value.is_some() {
            x += value.unwrap();
        }
    }

    let mut strengths_sum: i64 = 0;
    for strength in &strengths {
        strengths_sum += strength;
    }
    return strengths_sum;
}

fn main() {
    // example
    assert_eq!(13140, signal_strengths("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop"));

    assert_eq!(
        14320,
        signal_strengths("noop\nnoop\nnoop\naddx 6\nnoop\naddx 30\naddx -26\nnoop\naddx 5\nnoop\nnoop\nnoop\nnoop\naddx 5\naddx -5\naddx 6\naddx 5\naddx -1\naddx 5\nnoop\nnoop\naddx -14\naddx -18\naddx 39\naddx -39\naddx 25\naddx -22\naddx 2\naddx 5\naddx 2\naddx 3\naddx -2\naddx 2\nnoop\naddx 3\naddx 2\naddx 2\nnoop\naddx 3\nnoop\naddx 3\naddx 2\naddx 5\naddx 4\naddx -18\naddx 17\naddx -38\naddx 5\naddx 2\naddx -5\naddx 27\naddx -19\nnoop\naddx 3\naddx 4\nnoop\nnoop\naddx 5\naddx -1\nnoop\nnoop\naddx 4\naddx 5\naddx 2\naddx -4\naddx 5\nnoop\naddx -11\naddx 16\naddx -36\nnoop\naddx 5\nnoop\naddx 28\naddx -23\nnoop\nnoop\nnoop\naddx 21\naddx -18\nnoop\naddx 3\naddx 2\naddx 2\naddx 5\naddx 1\nnoop\nnoop\naddx 4\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 8\naddx -40\nnoop\naddx 7\nnoop\naddx -2\naddx 5\naddx 2\naddx 25\naddx -31\naddx 9\naddx 5\naddx 2\naddx 2\naddx 3\naddx -2\nnoop\naddx 3\naddx 2\nnoop\naddx 7\naddx -2\naddx 5\naddx -40\naddx 20\naddx -12\nnoop\nnoop\nnoop\naddx -5\naddx 7\naddx 7\nnoop\naddx -1\naddx 1\naddx 5\naddx 3\naddx -2\naddx 2\nnoop\naddx 3\naddx 2\nnoop\nnoop\nnoop\nnoop\naddx 7\nnoop\nnoop\nnoop\nnoop"));
}

/*
--- Day 10: Cathode-Ray Tube ---

You avoid the ropes, plunge into the river, and swim to shore.

The Elves yell something about meeting back up with them upriver, but the river is too loud to tell exactly what they're saying. They finish crossing the bridge and disappear from view.

Situations like this must be why the Elves prioritized getting the communication system on your handheld device working. You pull it out of your pack, but the amount of water slowly draining from a big crack in its screen tells you it probably won't be of much immediate use.

Unless, that is, you can design a replacement for the device's video system! It seems to be some kind of cathode-ray tube screen and simple CPU that are both driven by a precise clock circuit. The clock circuit ticks at a constant rate; each tick is called a cycle.

Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1. It supports only two instructions:

    addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
    noop takes one cycle to complete. It has no other effect.

The CPU uses these instructions in a program (your puzzle input) to, somehow, tell the screen what to draw.

Consider the following small program:

noop
addx 3
addx -5

Execution of this program proceeds as follows:

    At the start of the first cycle, the noop instruction begins execution. During the first cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
    At the start of the second cycle, the addx 3 instruction begins execution. During the second cycle, X is still 1.
    During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes execution, setting X to 4.
    At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth cycle, X is still 4.
    During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.

Maybe you can learn something by looking at the value of the X register throughout execution. For now, consider the signal strength (the cycle number multiplied by the value of the X register) during the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).

For example, consider this larger program:

addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop

The interesting signal strengths can be determined as follows:

    During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
    During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
    During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
    During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
    During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
    During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.

The sum of these signal strengths is 13140.

Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?
*/
