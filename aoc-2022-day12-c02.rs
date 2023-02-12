// https://adventofcode.com/2022/day/12
// (part 2)

struct Map<'a> {
    data: &'a str,
    width: usize,
    height: usize,
}

fn find_dimension(map: &str) -> (usize, usize) {
    let lines: Vec<&str> = map.split('\n').collect();

    return (lines[0].len() + 1, lines.len() - 1); // +1: include trailing '\n', -1 skip last empty line
}

fn find_first_location(map: &Map, charly: char) -> (usize, usize) {
    let index = map.data.find(charly).unwrap();

    return (index % map.width, index / map.width);
}

fn elevation_get(map: &Map, x: usize, y: usize) -> u8 {
    let index = y * map.width + x;
    let elevation = map.data.chars().nth(index).unwrap();

    match elevation {
        'S' => return 'a' as u8,
        'E' => return 'z' as u8,
        _ => return elevation as u8,
    }
}

fn update(
    current_loc: (usize, usize),
    x_shift: i8,
    y_shift: i8,
    map: &Map,
    distances: &mut Vec<usize>,
) -> bool {
    // skip out of bounds
    if x_shift < 0 && current_loc.0 == 0 {
        return false;
    }
    if x_shift > 0 && current_loc.0 == map.width - 2 {
        return false;
    }
    if y_shift < 0 && current_loc.1 == 0 {
        return false;
    }
    if y_shift > 0 && current_loc.1 == map.height - 1 {
        return false;
    }

    let current_elevation = elevation_get(map, current_loc.0, current_loc.1);
    let update_loc = (
        (current_loc.0 as i128 + x_shift as i128) as usize,
        (current_loc.1 as i128 + y_shift as i128) as usize,
    );
    let update_elevation: u8 = elevation_get(map, update_loc.0, update_loc.1);
    let elevation_diff = current_elevation as i16 - update_elevation as i16;
    if elevation_diff > 1 {
        // can't come from update_loc
        return false;
    }

    let current_index = current_loc.1 * map.width + current_loc.0;
    let current_distance = distances[current_index];
    let update_index = update_loc.1 * map.width + update_loc.0;
    let update_distance = distances[update_index];
    if update_distance != usize::MAX {
        // already seen
        return false;
    }

    distances[update_index] = current_distance + 1;
    return true;
}

fn map_distances(map: &Map, root_x: usize, root_y: usize, distances: &mut Vec<usize>) {
    distances.resize(map.width * map.height, usize::MAX);

    let mut distance: usize = 0;
    distances[root_y * map.width + root_x] = distance;

    let mut locations: Vec<(usize, usize)> = Vec::new();
    locations.push((root_x, root_y));

    // set distances from end
    while !locations.is_empty() {
        distance += 1;
        let current_locations = locations.to_vec(); // copy
        locations.clear();
        for loc in current_locations {
            // check left
            if update(loc, -1, 0, map, distances) {
                locations.push((loc.0 - 1, loc.1));
            }
            // check right
            if update(loc, 1, 0, map, distances) {
                locations.push((loc.0 + 1, loc.1));
            }
            // check top
            if update(loc, 0, -1, map, distances) {
                locations.push((loc.0, loc.1 - 1));
            }

            // check bot
            if update(loc, 0, 1, map, distances) {
                locations.push((loc.0, loc.1 + 1));
            }
        }
    }
}

fn find_shortest_path(input: &str) -> usize {
    let (map_width, map_height) = find_dimension(input);
    let map = Map {
        data: input,
        width: map_width,
        height: map_height,
    };

    let (end_x, end_y) = find_first_location(&map, 'E');

    let mut distances: Vec<usize> = Vec::new();
    map_distances(&map, end_x, end_y, &mut distances);

    let mut shortest_distance: usize = usize::MAX;
    for i in 0..map_height {
        for j in 0..map_width {
            let index = i * map_width + j;
            let c: char = input.chars().nth(index).unwrap();
            if c == 'a' || c == 'S' {
                let distance = distances[index];
                if distance < shortest_distance {
                    shortest_distance = distance;
                }
            }
        }
    }

    return shortest_distance;
}

fn main() {
    // example
    assert_eq!(
        29,
        find_shortest_path("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n")
    );
    // user puzzle
    assert_eq!(439, find_shortest_path("abccccccaaccaaccccaaaaacccccaaaaccccccccccccccccccccccccccccccccaaaaaaaaaaaaaaaaaaaccccccccccccccccaaaccccccccccccaacccccccccccccccccccccccccccccccccccccccccaaaa\nabaaaaccaaaaaccccaaaaaccccccaaaaccccccccccccccccccccaaacccccccccccaaaaaaaaaaaaaaaaaaccccccccccccccccaaaaccccccaaacaaccccccccccccccccccccccccccccccccccccccccaaaaa\nabaaacccaaaaaaaacaaaaaacccccaaaaccccccccccccccccccccaaaaacccccccccaaaaaaaaaaaaaaaaacccccccccaaccccaaaaaacccccccaaaaaccccaaccccccccccccccacccccccccccccccccccaaaaa\nabaaacccccaaaaaccccaaaaccccccaaacccccccccccccccccccaaaaaccccccccccaaaaaacacaaaaaacccccccccccaaccccaaaaacccccccccaaaaaaccaaaaaaccccccccccaaaccccacccccccccccaaaaaa\nabaacccccaaaaaccccaacccccccccccccccaaaaacccccccccccaaaaacccccccccaaaaaaaaccaaaaaaacccccccaaaaaaaaccaaaaacccccccaaaaaaaccaaaaacccccccccccaaacccaaaccccccccccccccaa\nabaaacccaaacaacccccccccccccccccccccaaaaaccccccccccccaaaaacccccccaaaaaaaaaccaaccaaacccccccaaaaaaaaccaacccccccccaaaaaaccaaaaaaccccccccccccaaaacaaaaccccccccccccccaa\nabaaacccccccaaccccccccccccccccccccaaaaaaccccccccccccaaccccccaacccaaaccaaaaccccccaacccccccccaaaacccccccccccccccaacaaaccaaaaaaaccccccccccccajjjjjjjcccccccccccccccc\nabcaacccccccccccccccccccccccccccccaaaaaaccccccccccccccccccccaaaaccccccaaaaccccccccccccccaacaaaaaccccccccccccccccccaaccccaaaaaacccccccccccjjjjjjjjjcccccaaaccccccc\nabccccccccccccccccccccccccccccccccaaaaaaccaaccccccccccccccaaaaaacccccccaaacccccccccccaacaaaaaaaaccccccccccccccccccccccccaaccaaccccccccaiijjjjojjjjcccccaaacaccccc\nabcccccccccccccccccccccccaaacccccccaaacacaaacccccccccccccccaaaaccccaaccccccccccccccccaaaaaaacccaccccccccccccccccccccccccaacccccccccccaiiijjooooojjkccaaaaaaaacccc\nabccccccccccccccccccccccaaaaccccccccccaaaaaccccccccccccccccaaaaacccaaaaaccccccccccccccaaaaaacccccccccccccccccccccccccccccccccccccciiiiiiiioooooookkkcaaaaaaaacccc\nabccccccccccccccccccccccaaaaccccccccccaaaaaaaacccccccccccccaacaaccaaaaacccccccaaacccaaaaaaaaccccccccccccccccccccccccccccccccccchiiiiiiiiooooouoookkkccaaaaaaccccc\nabcccccccccaaccccccccccccaaaccccccccccccaaaaacccccccccccccccccccccaaaaaccccccaaaacccaaaaacaacccccccccccccaacaacccccccccccccccchhhiiiinnnooouuuuoookkkccaaaaaccccc\nabcccccccccaaacccccccccccccccccccccccccaaaaacccccccccccccccccccccccaaaaacccccaaaaccccccaaccccccccccccccccaaaaacccccccccccccccchhhnnnnnnnnouuuuuuppkkkkaaaaaaccccc\nabccccccaaaaaaaacccaaccccccccccccccccccaacaaccaacaaccccccccccccccccaacccccccccaaaccccccaacccccccccccccccaaaaacccccccccccccccchhhnnnnnnnnntuuxuuupppkkkkkacccccccc\nabccccccaaaaaaaacacaaaacccccccccccccccccccaaccaaaaacccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaaacccccccaacccccchhhnnnnttttttuxxxuuppppkkkkkcccccccc\nabcccccccaaaaaaccaaaaaaccccccccccaaccccccccccaaaaaccccccccccccccccccccccccaaacccccccccccccccccccccccccccccaaaaccaaccaaacccaaahhhnnntttttttuxxxxuupppppllllccccccc\nabcccccccaaaaaacccaaaacccccccccaaaaaaccccccccaaaaaacccccccccccccccccccccccaaacccccccccccccccccccccccccccccacccccaaaaaaacaaaaahhhppntttxxxxxxxxuuuuvpppplllccccccc\nabcccccccaaaaaacccaaaacccccccccaaaaaacccccaaaaaaaaaccccccccccccccccccccaaaaaaaacccccccccccccccccccccaaaccccccaacaaaaaaccaaaaahhhpppttxxxxxxxxyuuvvvvvppplllcccccc\nabcccccccaaccaacccaacaccaaaaccccaaaaacccccaaaaaaaaaccccccccccccccccccccaaaaaaaacccccccccccccccccccccaaacaaaaaaaccaaaaaaaaaaaaahhppptttxxxxxxyyyyyyvvvppplllcccccc\nSbccccccccccccccccccccccaaaacccaaaaacccccccaaaaaaaaacaaaccccccccaacccccccaaaaaccccccccaaaaacccccccccaaaaaaaaaaaaaaaaaaaaaaaaacgggpppttxxxxEzzyyyyyvvvqqqlllcccccc\nabccccccccccccccccccccccaaaacccaaaaacccccccaaaaaaaaccaaaccccccccaaacaaccaaaaaaccccccccaaaaacccccccaaaaaaaaaaaaaaaaaaaaaaaaaaacgggpppsssxxxyyyyyyvvvvvqqlllccccccc\nabcccaaaccccccccccccccccaaaccccccccccccccccaaaaaaaaaaaaaccccccccaaaaaaccaaaaaacccccccaaaaaacccaaaccaaaaaccaaaaaaaaaaaacccccccccgggppssswwyyyyyyvvvvqqqqlllccccccc\nabcaaaaaccccccccccccccccccccccccccccccccccaaaaaaaaaaaaacccccccaaaaaaacccaccaaacccccccaaaaaacccaaacccaaaaaaaaaaaccccaaacccaaaaacgggppsswwwyyyyyyvvqqqqqlllcccccccc\nabcaaaaaaccccccccccccccccccccccccccccccccccaaccaaaaaaaaaaaccccaaaaaaacccccccccccccccccaaaaacccaaacaaaacaaaaaaaaccccaaacccaaaaacggpppsswwwywwyyyvvqqqmmmlccccccccc\nabcaaaaaacccccccaacaaccccccccccccccccccccccccccaaaaaaaaaaaccccccaaaaacccccccccccccccccaaaccaaaaaaaaaaacccccccaacccccccccaaaaaacggpppsswwwwwwwwyvvqqqmmmcccccccccc\nabcaaaaaccccccccaaaaaccccccccccccccccccccccccccccaaaaaaaacccccccaacaaacccccccccccccccccccccaaaaaaaaaccccccccccccccccccccaaaaaagggoossswwwwrrwwwvvqqmmmccccccccccc\nabcaaaaacccccccaaaaaccccccccccccccccccccccccccccaaaaaaacccccccccaaccccccccccccccccccccccccccaaaaaaacccccccccccaaaccccccccaaaaagggooosssssrrrrwwwvqqmmmcccaacccccc\nabcccccccccccccaaaaaaccccccccccccccccccccaacccccccccaaaccccccccccccccccccccccccccccccccccccccaaaaaaccccccccccccaaaaccccccaaaccgggooosssssrrrrrwwrrqmmmcccaacccccc\nabcccccccccccccccaaaacccccccccccccccccccaaaacccccccacaaacccccccccccccccccccccccccccccccccccccaaaaaaacccccccccaaaaaacccccccccccgffoooooosoonrrrrrrrrmmmccaaaaacccc\nabcccccccccccccccaccccccccccccccccccccccaaaacccccccaaaaacccccccccccccccccccccccccccccccccccccaaacaaacccccccccaaaaacccccccccccccfffoooooooonnnrrrrrmmmddcaaaaacccc\nabccccccccccccccccccccccccccccccccccccccaaaaccccccccaaaaacccccccccccccccccccccccccaaaccccccccaacccccccccccccccaaaaaccccccccccccffffoooooonnnnnnrnnmmmdddaaaaacccc\nabcccccccccccccccccccccccccccccccccccccccccccccccccaaaaaacccccccccccccccccaaaaaccaaaacccccccccccccccccccccccccaacccccccccccccccfffffffffeeeennnnnnmmdddaaaacccccc\nabcccccccaaaccccccccaccccccccccccccccccccccccccccccaaaaccccccccccccaaaccccaaaaaccaaaaccccccccccccccccccccccccccccccccccccccccccccfffffffeeeeennnnnmddddaaaaaccccc\nabcccaaccaaacccccaaaacccccaacccccccccccccccccccccccccaaacccccccccccaaacccaaaaaacccaaaccccccccccccccccccccccccccccccccccccccccccccccffffeeeeeeeedddddddcccaacccccc\nabcccaaaaaaacccccaaaaaaccaaacccccccccccccccccccccccccccacccccccccccaaaaccaaaaaaccccccccccccccccccccccccccaacccccccccaaaccccccccccccccaaaaaaeeeeedddddcccccccccccc\nabcccaaaaaacccccccaaaacccaaacaaaccccaaaacccccccccaaacaaaccccccaacccaaaacaaaaaaacccccccccccccccccccccccccaaaccccccccaaaacccccccccccccccccccaaaaeeddddccccccccccccc\nabccccaaaaaaaacccaaaaaaaaaaaaaaaccccaaaacccccccccaaaaaaacccccaaacccaaaaaaaaaacccccccccccccccccccccccaaacaaaccccccccaaaaccccccccccccccccccccaaaccccccccccccccaaaca\nabcccaaaaaaaaacccaacaaaaaaaaaaacccccaaaaccccccccccaaaaaacaaacaaacaaaaaaaaaacccccccccccccccaaacccccccaaaaaaaaaaccccccaaaccccccccccccccccccccaacccccccccccccccaaaaa\nabcccaaaaaaaacccccccccccaaaaaacccccccaacccccccccccaaaaaaaaaaaaaaaaaaaaaaaaccccccccccccccccaaaacccccccaaaaaaaaacccccccccccccccccccccccccccccaaacccccccccccccccaaaa\nabccaaaaaaacccccccccccccaaaaaaacccccccccccccccccaaaaaaaaaaaaaaaaaaaaaaaaaaacccccccccccccccaaaacccccccaaaaaaaacccccccccccccccccccccccccccccccccccccccccccccccaaaaa\n"));
}

/*
--- Part Two ---

As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from any square at elevation a to the square marked E.

Again consider the example from above:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^

This path reaches the goal in only 29 steps, the fewest possible.

What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?
*/
