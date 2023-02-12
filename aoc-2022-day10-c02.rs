// https://adventofcode.com/2022/day/10
// (part 2)

const COMMAND_ADD: &str = "addx ";
const X_START_VALUE: i64 = 1;
const CRT_COLUMNS_COUNT: usize = 40;
const CRT_ROWS_COUNT: usize = 6;
const PIXEL_LIT: u8 = '#' as u8;
const PIXEL_DARK: u8 = ' ' as u8;

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

fn draw(input: &str) {
    let mut crt: [u8; CRT_COLUMNS_COUNT * CRT_ROWS_COUNT] =
        [PIXEL_DARK; CRT_COLUMNS_COUNT * CRT_ROWS_COUNT];

    let commands: Vec<&str> = input.split('\n').collect();
    let mut x: i64 = X_START_VALUE;
    let mut cycle_index: usize = 0;
    for command in commands {
        let mut value: Option<i64> = Option::None;
        let mut command_cycles_count: usize = 0;
        if command == "noop" {
            command_cycles_count = 1;
        } else if command.starts_with(COMMAND_ADD) {
            let (add_value, chars_read_count) =
                number_parse(command.get(COMMAND_ADD.len()..).unwrap());
            assert!(command.len() == chars_read_count + COMMAND_ADD.len());
            command_cycles_count = 2;
            value = Some(add_value);
        } else {
            assert!(false);
        }

        for _ in 0..command_cycles_count {
            let col_index: i64 = (cycle_index % CRT_COLUMNS_COUNT) as i64;
            let pixel: u8 = if col_index >= x - 1 && col_index as i64 <= x + 1 {
                PIXEL_LIT
            } else {
                PIXEL_DARK
            };
            crt[cycle_index] = pixel;
            cycle_index += 1;
        }

        if value.is_some() {
            x += value.unwrap();
        }
    }

    for i in 0..CRT_ROWS_COUNT {
        let row =
            std::str::from_utf8(&crt[i * CRT_COLUMNS_COUNT..(i + 1) * CRT_COLUMNS_COUNT]).unwrap();
        print!("{}\n", row);
    }
}

fn main() {
    // example
    draw("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
    print!("\n");
    draw("noop\nnoop\nnoop\naddx 6\nnoop\naddx 30\naddx -26\nnoop\naddx 5\nnoop\nnoop\nnoop\nnoop\naddx 5\naddx -5\naddx 6\naddx 5\naddx -1\naddx 5\nnoop\nnoop\naddx -14\naddx -18\naddx 39\naddx -39\naddx 25\naddx -22\naddx 2\naddx 5\naddx 2\naddx 3\naddx -2\naddx 2\nnoop\naddx 3\naddx 2\naddx 2\nnoop\naddx 3\nnoop\naddx 3\naddx 2\naddx 5\naddx 4\naddx -18\naddx 17\naddx -38\naddx 5\naddx 2\naddx -5\naddx 27\naddx -19\nnoop\naddx 3\naddx 4\nnoop\nnoop\naddx 5\naddx -1\nnoop\nnoop\naddx 4\naddx 5\naddx 2\naddx -4\naddx 5\nnoop\naddx -11\naddx 16\naddx -36\nnoop\naddx 5\nnoop\naddx 28\naddx -23\nnoop\nnoop\nnoop\naddx 21\naddx -18\nnoop\naddx 3\naddx 2\naddx 2\naddx 5\naddx 1\nnoop\nnoop\naddx 4\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 8\naddx -40\nnoop\naddx 7\nnoop\naddx -2\naddx 5\naddx 2\naddx 25\naddx -31\naddx 9\naddx 5\naddx 2\naddx 2\naddx 3\naddx -2\nnoop\naddx 3\naddx 2\nnoop\naddx 7\naddx -2\naddx 5\naddx -40\naddx 20\naddx -12\nnoop\nnoop\nnoop\naddx -5\naddx 7\naddx 7\nnoop\naddx -1\naddx 1\naddx 5\naddx 3\naddx -2\naddx 2\nnoop\naddx 3\naddx 2\nnoop\nnoop\nnoop\nnoop\naddx 7\nnoop\nnoop\nnoop\nnoop");
}

/*
--- Part Two ---

It seems like the X register controls the horizontal position of a sprite. Specifically, the sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that sprite. (In this system, there is no such thing as "vertical position": if the sprite's horizontal position puts its pixels where the CRT is currently drawing, then those pixels will be drawn.)

You count the pixels on the CRT: 40 wide and 6 high. This CRT screen draws the top row of pixels left-to-right, then the row below that, and so on. The left-most pixel in each row is in position 0, and the right-most pixel in each row is in position 39.

Like the CPU, the CRT is tied closely to the clock circuit: the CRT draws a single pixel during each cycle. Representing each pixel of the screen as a #, here are the cycles during which the first and last pixel in each row are drawn:

Cycle   1 -> ######################################## <- Cycle  40
Cycle  41 -> ######################################## <- Cycle  80
Cycle  81 -> ######################################## <- Cycle 120
Cycle 121 -> ######################################## <- Cycle 160
Cycle 161 -> ######################################## <- Cycle 200
Cycle 201 -> ######################################## <- Cycle 240

So, by carefully timing the CPU instructions and the CRT drawing operations, you should be able to determine whether the sprite is visible the instant each pixel is drawn. If the sprite is positioned such that one of its three pixels is the pixel currently being drawn, the screen produces a lit pixel (#); otherwise, the screen leaves the pixel dark (.).

The first few pixels from the larger example above are drawn as follows:

Sprite position: ###.....................................

Start cycle   1: begin executing addx 15
During cycle  1: CRT draws pixel in position 0
Current CRT row: #

During cycle  2: CRT draws pixel in position 1
Current CRT row: ##
End of cycle  2: finish executing addx 15 (Register X is now 16)
Sprite position: ...............###......................

Start cycle   3: begin executing addx -11
During cycle  3: CRT draws pixel in position 2
Current CRT row: ##.

During cycle  4: CRT draws pixel in position 3
Current CRT row: ##..
End of cycle  4: finish executing addx -11 (Register X is now 5)
Sprite position: ....###.................................

Start cycle   5: begin executing addx 6
During cycle  5: CRT draws pixel in position 4
Current CRT row: ##..#

During cycle  6: CRT draws pixel in position 5
Current CRT row: ##..##
End of cycle  6: finish executing addx 6 (Register X is now 11)
Sprite position: ..........###...........................

Start cycle   7: begin executing addx -3
During cycle  7: CRT draws pixel in position 6
Current CRT row: ##..##.

During cycle  8: CRT draws pixel in position 7
Current CRT row: ##..##..
End of cycle  8: finish executing addx -3 (Register X is now 8)
Sprite position: .......###..............................

Start cycle   9: begin executing addx 5
During cycle  9: CRT draws pixel in position 8
Current CRT row: ##..##..#

During cycle 10: CRT draws pixel in position 9
Current CRT row: ##..##..##
End of cycle 10: finish executing addx 5 (Register X is now 13)
Sprite position: ............###.........................

Start cycle  11: begin executing addx -1
During cycle 11: CRT draws pixel in position 10
Current CRT row: ##..##..##.

During cycle 12: CRT draws pixel in position 11
Current CRT row: ##..##..##..
End of cycle 12: finish executing addx -1 (Register X is now 12)
Sprite position: ...........###..........................

Start cycle  13: begin executing addx -8
During cycle 13: CRT draws pixel in position 12
Current CRT row: ##..##..##..#

During cycle 14: CRT draws pixel in position 13
Current CRT row: ##..##..##..##
End of cycle 14: finish executing addx -8 (Register X is now 4)
Sprite position: ...###..................................

Start cycle  15: begin executing addx 13
During cycle 15: CRT draws pixel in position 14
Current CRT row: ##..##..##..##.

During cycle 16: CRT draws pixel in position 15
Current CRT row: ##..##..##..##..
End of cycle 16: finish executing addx 13 (Register X is now 17)
Sprite position: ................###.....................

Start cycle  17: begin executing addx 4
During cycle 17: CRT draws pixel in position 16
Current CRT row: ##..##..##..##..#

During cycle 18: CRT draws pixel in position 17
Current CRT row: ##..##..##..##..##
End of cycle 18: finish executing addx 4 (Register X is now 21)
Sprite position: ....................###.................

Start cycle  19: begin executing noop
During cycle 19: CRT draws pixel in position 18
Current CRT row: ##..##..##..##..##.
End of cycle 19: finish executing noop

Start cycle  20: begin executing addx -1
During cycle 20: CRT draws pixel in position 19
Current CRT row: ##..##..##..##..##..

During cycle 21: CRT draws pixel in position 20
Current CRT row: ##..##..##..##..##..#
End of cycle 21: finish executing addx -1 (Register X is now 20)
Sprite position: ...................###..................

Allowing the program to run to completion causes the CRT to produce the following image:

##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....

Render the image given by your program. What eight capital letters appear on your CRT?
*/
