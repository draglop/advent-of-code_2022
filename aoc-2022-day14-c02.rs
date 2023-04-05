// https://adventofcode.com/2022/day/14
// (part 2)

const SAND_DROP_X: usize = 500;
const SAND_DROP_Y: usize = 0;

struct Map {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

#[derive(PartialEq)]
enum MoveStatus {
    Moved,
    Stuck,
    OutOfBounds,
}

struct Point {
    x: usize,
    y: usize,
}

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

fn generate_lines(input: &str) -> Vec<Vec<Point>> {
    let input_lines: Vec<&str> = input.split('\n').collect();
    let mut rock_lines: Vec<Vec<Point>> = Vec::with_capacity(input_lines.len());

    for input_line in input_lines {
        let mut rock_line: Vec<Point> = Vec::with_capacity(32);

        let mut offset: usize = 0;
        while offset < input_line.len() {
            let (x, x_size) = number_parse(input_line.get(offset..).unwrap());
            assert!(x_size > 0);
            assert!(input_line.chars().nth(x_size) == Some(','));
            offset += x_size + 1;
            let (y, y_size) = number_parse(input_line.get(offset..).unwrap());
            assert!(y_size > 0);
            offset += y_size;
            assert!(x > 0);
            assert!(y > 0);
            rock_line.push(Point {
                x: x as usize,
                y: y as usize,
            });
            offset += " -> ".len(); // would be safer to assert that data is a match
        }

        rock_lines.push(rock_line);
    }

    return rock_lines;
}

fn find_dimensions(lines: &Vec<Vec<Point>>) -> (usize, usize) {
    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in lines {
        for point in line {
            if point.x > width {
                width = point.x
            }
            if point.y > height {
                height = point.y
            }
        }
    }

    return (width + 1, height + 1);
}

fn draw_lines(map: &mut Map, liness: &Vec<Vec<Point>>) {
    for lines in liness {
        assert!(lines.len() >= 2);
        let mut from = &lines[0];
        for i in 1..lines.len() {
            let to = &lines[i];
            if from.x == to.x {
                let y_start = if to.y >= from.y { from.y } else { to.y };
                let y_end = if to.y >= from.y { to.y } else { from.y };
                for y in y_start..y_end + 1 {
                    map.data[y * map.width + from.x] = true;
                }
            } else if from.y == to.y {
                let x_start = if to.x >= from.x { from.x } else { to.x };
                let x_end = if to.x >= from.x { to.x } else { from.x };
                for x in x_start..x_end + 1 {
                    map.data[from.y * map.width + x] = true;
                }
            } else {
                panic!("not a horizontal nor a vertical line");
            }

            from = to;
        }
    }
}

fn generate_map(input: &str) -> Map {
    let liness: Vec<Vec<Point>> = generate_lines(input);
    let (mut width, mut height) = find_dimensions(&liness);
    height += 1; // extra line of stuck pieces
    width = if width > SAND_DROP_X + height {
        width
    } else {
        SAND_DROP_X + height
    };

    let mut map: Map = Map {
        data: Vec::new(),
        width: width,
        height: height,
    };
    map.data.resize_with(map.width * map.height, || false);

    draw_lines(&mut map, &liness);

    return map;
}

fn move_sand(map: &Map, sand: &mut Point) -> MoveStatus {
    let candidate_y = sand.y + 1;
    if candidate_y == map.height {
        return MoveStatus::Stuck;
    }

    let mut candidate_x = sand.x;
    let mut candidate_offset = candidate_y * map.width + candidate_x;
    if map.data[candidate_offset] == true {
        // try left
        if candidate_x == 0 {
            return MoveStatus::OutOfBounds;
        }
        candidate_x -= 1;
        candidate_offset -= 1;
        if map.data[candidate_offset] == true {
            // try right
            if candidate_x == map.width - 1 {
                return MoveStatus::OutOfBounds;
            }
            candidate_x += 2;
            candidate_offset += 2;
        }
    }

    if map.data[candidate_offset] == false {
        sand.x = candidate_x;
        sand.y = candidate_y;
        return MoveStatus::Moved;
    } else {
        return MoveStatus::Stuck;
    }
}

fn count_sand(input: &str) -> usize {
    let mut sand_count = 0;

    let mut map: Map = generate_map(input);

    loop {
        let mut sand = Point {
            x: SAND_DROP_X,
            y: SAND_DROP_Y,
        };
        sand_count += 1;

        let mut move_status: MoveStatus = move_sand(&map, &mut sand);
        if move_status == MoveStatus::Stuck {
            break;
        }

        while move_status == MoveStatus::Moved {
            move_status = move_sand(&map, &mut sand);
        }

        assert!(move_status != MoveStatus::OutOfBounds);
        let offset = sand.y * map.width + sand.x;
        assert!(map.data[offset] == false);
        map.data[offset] = true;
    }

    return sand_count;
}

fn main() {
    // example
    assert_eq!(
        93,
        count_sand("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9")
    );
    // user puzzle
    assert_eq!(27155, count_sand("506,104 -> 511,104\n504,96 -> 509,96\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n499,35 -> 504,35\n511,23 -> 516,23\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n503,38 -> 508,38\n520,104 -> 525,104\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n499,28 -> 499,29 -> 513,29 -> 513,28\n504,23 -> 509,23\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n499,28 -> 499,29 -> 513,29 -> 513,28\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n508,98 -> 513,98\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n507,20 -> 512,20\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n510,127 -> 514,127\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n492,104 -> 497,104\n492,35 -> 497,35\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n528,127 -> 532,127\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n516,127 -> 520,127\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n495,32 -> 500,32\n493,14 -> 505,14 -> 505,13\n522,127 -> 526,127\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n493,14 -> 505,14 -> 505,13\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n525,125 -> 529,125\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n513,104 -> 518,104\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n519,121 -> 523,121\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n495,102 -> 500,102\n500,20 -> 505,20\n490,42 -> 504,42 -> 504,41\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n509,102 -> 514,102\n513,125 -> 517,125\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n490,42 -> 504,42 -> 504,41\n498,100 -> 503,100\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n489,38 -> 494,38\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n496,38 -> 501,38\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n519,125 -> 523,125\n501,154 -> 501,155 -> 507,155\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n499,104 -> 504,104\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n501,154 -> 501,155 -> 507,155\n503,17 -> 508,17\n516,123 -> 520,123\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n505,100 -> 510,100\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n500,93 -> 500,83 -> 500,93 -> 502,93 -> 502,90 -> 502,93 -> 504,93 -> 504,85 -> 504,93 -> 506,93 -> 506,83 -> 506,93\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,140 -> 498,142 -> 493,142 -> 493,150 -> 504,150 -> 504,142 -> 503,142 -> 503,140\n506,130 -> 506,132 -> 502,132 -> 502,137 -> 516,137 -> 516,132 -> 511,132 -> 511,130\n516,102 -> 521,102\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n522,123 -> 526,123\n485,45 -> 485,47 -> 483,47 -> 483,54 -> 492,54 -> 492,47 -> 489,47 -> 489,45\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n499,28 -> 499,29 -> 513,29 -> 513,28\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n512,100 -> 517,100\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n497,23 -> 502,23\n524,107 -> 524,111 -> 522,111 -> 522,118 -> 534,118 -> 534,111 -> 528,111 -> 528,107\n489,67 -> 489,58 -> 489,67 -> 491,67 -> 491,61 -> 491,67 -> 493,67 -> 493,59 -> 493,67 -> 495,67 -> 495,64 -> 495,67\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n501,98 -> 506,98\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n498,168 -> 498,158 -> 498,168 -> 500,168 -> 500,162 -> 500,168 -> 502,168 -> 502,163 -> 502,168 -> 504,168 -> 504,160 -> 504,168 -> 506,168 -> 506,159 -> 506,168 -> 508,168 -> 508,163 -> 508,168 -> 510,168 -> 510,163 -> 510,168 -> 512,168 -> 512,167 -> 512,168 -> 514,168 -> 514,161 -> 514,168 -> 516,168 -> 516,167 -> 516,168\n502,102 -> 507,102\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80\n487,80 -> 487,75 -> 487,80 -> 489,80 -> 489,72 -> 489,80 -> 491,80 -> 491,76 -> 491,80 -> 493,80 -> 493,71 -> 493,80 -> 495,80 -> 495,76 -> 495,80 -> 497,80 -> 497,75 -> 497,80 -> 499,80 -> 499,70 -> 499,80 -> 501,80 -> 501,78 -> 501,80 -> 503,80 -> 503,70 -> 503,80"
            ));
}

/*
--- Part Two ---

You realize you misread the scan. There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!

You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan.

In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11. (This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.) With the added floor, the example above now looks like this:

        ...........+........
        ....................
        ....................
        ....................
        .........#...##.....
        .........#...#......
        .......###...#......
        .............#......
        .............#......
        .....#########......
        ....................
<-- etc #################### etc -->

To find somewhere safe to stand, you'll need to simulate falling sand until a unit of sand comes to rest at 500,0, blocking the source entirely and stopping the flow of sand into the cave. In the example above, the situation finally looks like this after 93 units of sand come to rest:

............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################

Using your scan, simulate the falling sand until the source of the sand becomes blocked. How many units of sand come to rest?
*/