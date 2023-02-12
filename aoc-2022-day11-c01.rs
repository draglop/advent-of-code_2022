// https://adventofcode.com/2022/day/11
// (part 1)

const MONKEY_LINE_ID_PREFIX: &str = "Monkey ";
const MONKEY_LINE_ITEMS_PREFIX: &str = "  Starting items: ";
const MONKEY_LINE_OPERATION_PREFIX: &str = "  Operation: new = old ";
const MONKEY_LINE_TEST_PREFIX: &str = "  Test: divisible by ";
const MONKEY_LINE_IFFALSE_PREFIX: &str = "    If false: throw to monkey ";
const MONKEY_LINE_IFTRUE_PREFIX: &str = "    If true: throw to monkey ";
const RELIEF_FACTOR: usize = 3;
const ROUNDS_COUNT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    NONE,
    ADD,
    MULTIPLY,
    SQUARE,
}

#[derive(Clone, Copy)]
struct MonkeyOperation {
    operator: Operator,
    operand: usize,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: MonkeyOperation,
    divisible_by: usize,
    throw_to_if_false: usize,
    throw_to_if_true: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: Vec::new(),
            operation: MonkeyOperation {
                operator: Operator::NONE,
                operand: usize::MAX,
            },
            divisible_by: usize::MAX,
            throw_to_if_true: usize::MAX,
            throw_to_if_false: usize::MAX,
        }
    }
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

fn update_worry_level(mut worry_level: usize, operation: MonkeyOperation, relief: usize) -> usize {
    match operation.operator {
        Operator::ADD => worry_level += operation.operand,
        Operator::MULTIPLY => worry_level *= operation.operand,
        Operator::SQUARE => worry_level *= worry_level,
        _ => panic!("expected MonkeyOperation"),
    }

    worry_level /= relief;

    return worry_level;
}

fn monkeys_parse(input: &str, monkeys: &mut Vec<Monkey>) {
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines {
        if line.is_empty() {
            // skip
        } else if line.starts_with(MONKEY_LINE_ID_PREFIX) {
            // new monkey
            let monkey: Monkey = Monkey::new();
            let (monkey_index, _digit_count) =
                number_parse(line.get(MONKEY_LINE_ID_PREFIX.len()..).unwrap());
            assert!(monkey_index == monkeys.len() as i64);
            monkeys.push(monkey);
        } else {
            // update current monkey
            let monkey: &mut Monkey = monkeys.last_mut().unwrap();
            if line.starts_with(MONKEY_LINE_ITEMS_PREFIX) {
                assert!(monkey.items.is_empty());
                let mut items_option: Option<&str> = line.get(MONKEY_LINE_ITEMS_PREFIX.len()..);
                while !items_option.is_none() {
                    let items_str = items_option.unwrap();
                    let (item, digit_count) = number_parse(items_str);
                    monkey.items.push(item as usize);
                    items_option = items_str.get(digit_count + 2..); // consume ", "
                }
            } else if line.starts_with(MONKEY_LINE_OPERATION_PREFIX) {
                assert!(monkey.operation.operator == Operator::NONE);
                let operation_str = line.get(MONKEY_LINE_OPERATION_PREFIX.len()..).unwrap();
                if operation_str == "* old" {
                    monkey.operation.operator = Operator::SQUARE;
                } else {
                    if operation_str.chars().nth(0).unwrap() == '*' {
                        monkey.operation.operator = Operator::MULTIPLY;
                    } else if operation_str.chars().nth(0).unwrap() == '+' {
                        monkey.operation.operator = Operator::ADD;
                    } else {
                        panic!(
                            "unexpected operation [{}] in line [{}]",
                            operation_str, line
                        );
                    }
                    let (operand, digit_count) = number_parse(operation_str.get(2..).unwrap());
                    assert!(digit_count > 0);
                    monkey.operation.operand = operand as usize;
                }
            } else if line.starts_with(MONKEY_LINE_TEST_PREFIX) {
                assert!(monkey.divisible_by == usize::MAX);
                let (divisible_by, digit_count) =
                    number_parse(line.get(MONKEY_LINE_TEST_PREFIX.len()..).unwrap());
                assert!(digit_count > 0);
                monkey.divisible_by = divisible_by as usize;
            } else if line.starts_with(MONKEY_LINE_IFFALSE_PREFIX) {
                assert!(monkey.throw_to_if_false == usize::MAX);
                let (monkey_index, digit_count) =
                    number_parse(line.get(MONKEY_LINE_IFFALSE_PREFIX.len()..).unwrap());
                assert!(digit_count > 0);
                monkey.throw_to_if_false = monkey_index as usize;
            } else if line.starts_with(MONKEY_LINE_IFTRUE_PREFIX) {
                assert!(monkey.throw_to_if_true == usize::MAX);
                let (monkey_index, digit_count) =
                    number_parse(line.get(MONKEY_LINE_IFTRUE_PREFIX.len()..).unwrap());
                assert!(digit_count > 0);
                monkey.throw_to_if_true = monkey_index as usize;
            } else {
                panic!("unexpected line [{}]", line);
            }
        }
    }
}

fn monkey_business_level(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys_parse(&input, &mut monkeys);

    let mut inspected_counts: Vec<usize> = Vec::with_capacity(monkeys.len());
    inspected_counts.resize(monkeys.len(), 0);

    for _ in 0..ROUNDS_COUNT {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone(); // pfff !
            for item in &monkey.items {
                let worry_level = update_worry_level(*item, monkey.operation, RELIEF_FACTOR);
                let reminder = worry_level % monkey.divisible_by;
                if reminder == 0 {
                    monkeys[monkey.throw_to_if_true].items.push(worry_level);
                } else {
                    monkeys[monkey.throw_to_if_false].items.push(worry_level);
                }
            }
            inspected_counts[i] += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
    inspected_counts.sort();

    return inspected_counts[inspected_counts.len() - 1]
        * inspected_counts[inspected_counts.len() - 2];
}

fn main() {
    // example
    assert_eq!(10605, monkey_business_level(
"Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1"));
    // user puzzle
    assert_eq!(117640, monkey_business_level("Monkey 0:\n  Starting items: 63, 84, 80, 83, 84, 53, 88, 72\n  Operation: new = old * 11\n  Test: divisible by 13\n    If true: throw to monkey 4\n    If false: throw to monkey 7\n\nMonkey 1:\n  Starting items: 67, 56, 92, 88, 84\n  Operation: new = old + 4\n  Test: divisible by 11\n    If true: throw to monkey 5\n    If false: throw to monkey 3\n\nMonkey 2:\n  Starting items: 52\n  Operation: new = old * old\n  Test: divisible by 2\n    If true: throw to monkey 3\n    If false: throw to monkey 1\n\nMonkey 3:\n  Starting items: 59, 53, 60, 92, 69, 72\n  Operation: new = old + 2\n  Test: divisible by 5\n    If true: throw to monkey 5\n    If false: throw to monkey 6\n\nMonkey 4:\n  Starting items: 61, 52, 55, 61\n  Operation: new = old + 3\n  Test: divisible by 7\n    If true: throw to monkey 7\n    If false: throw to monkey 2\n\nMonkey 5:\n  Starting items: 79, 53\n  Operation: new = old + 1\n  Test: divisible by 3\n    If true: throw to monkey 0\n    If false: throw to monkey 6\n\nMonkey 6:\n  Starting items: 59, 86, 67, 95, 92, 77, 91\n  Operation: new = old + 5\n  Test: divisible by 19\n    If true: throw to monkey 4\n    If false: throw to monkey 0\n\nMonkey 7:\n  Starting items: 58, 83, 89\n  Operation: new = old * 19\n  Test: divisible by 17\n    If true: throw to monkey 2\n    If false: throw to monkey 1"));
}

/*
--- Day 11: Monkey in the Middle ---

As you finally start making your way upriver, you realize your pack is much lighter than you remember. Just then, one of the items from your pack goes flying overhead. Monkeys are playing Keep Away with your missing things!

To get your stuff back, you need to be able to predict where the monkeys will throw your items. After some careful observation, you realize the monkeys operate based on how worried you are about each item.

You take some notes (your puzzle input) on the items each monkey currently has, how worried you are about those items, and how the monkey makes decisions based on your worry level. For example:

Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

Each monkey has several attributes:

    Starting items lists your worry level for each item the monkey is currently holding in the order they will be inspected.
    Operation shows how your worry level changes as that monkey inspects an item. (An operation like new = old * 5 means that your worry level after the monkey inspected the item is five times whatever your worry level was before inspection.)
    Test shows how the monkey uses your worry level to decide where to throw an item next.
        If true shows what happens with an item if the Test was true.
        If false shows what happens with an item if the Test was false.

After each monkey inspects an item but before it tests your worry level, your relief that the monkey's inspection didn't damage the item causes your worry level to be divided by three and rounded down to the nearest integer.

The monkeys take turns inspecting and throwing items. On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed. Monkey 0 goes first, then monkey 1, and so on until each monkey has had one turn. The process of each monkey taking a single turn is called a round.

When a monkey throws an item to another monkey, the item goes on the end of the recipient monkey's list. A monkey that starts a round with no items could end up inspecting and throwing many items by the time its turn comes around. If a monkey is holding no items at the start of its turn, its turn ends.

In the above example, the first round proceeds as follows:

Monkey 0:
  Monkey inspects an item with a worry level of 79.
    Worry level is multiplied by 19 to 1501.
    Monkey gets bored with item. Worry level is divided by 3 to 500.
    Current worry level is not divisible by 23.
    Item with worry level 500 is thrown to monkey 3.
  Monkey inspects an item with a worry level of 98.
    Worry level is multiplied by 19 to 1862.
    Monkey gets bored with item. Worry level is divided by 3 to 620.
    Current worry level is not divisible by 23.
    Item with worry level 620 is thrown to monkey 3.
Monkey 1:
  Monkey inspects an item with a worry level of 54.
    Worry level increases by 6 to 60.
    Monkey gets bored with item. Worry level is divided by 3 to 20.
    Current worry level is not divisible by 19.
    Item with worry level 20 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 65.
    Worry level increases by 6 to 71.
    Monkey gets bored with item. Worry level is divided by 3 to 23.
    Current worry level is not divisible by 19.
    Item with worry level 23 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 75.
    Worry level increases by 6 to 81.
    Monkey gets bored with item. Worry level is divided by 3 to 27.
    Current worry level is not divisible by 19.
    Item with worry level 27 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 74.
    Worry level increases by 6 to 80.
    Monkey gets bored with item. Worry level is divided by 3 to 26.
    Current worry level is not divisible by 19.
    Item with worry level 26 is thrown to monkey 0.
Monkey 2:
  Monkey inspects an item with a worry level of 79.
    Worry level is multiplied by itself to 6241.
    Monkey gets bored with item. Worry level is divided by 3 to 2080.
    Current worry level is divisible by 13.
    Item with worry level 2080 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 60.
    Worry level is multiplied by itself to 3600.
    Monkey gets bored with item. Worry level is divided by 3 to 1200.
    Current worry level is not divisible by 13.
    Item with worry level 1200 is thrown to monkey 3.
  Monkey inspects an item with a worry level of 97.
    Worry level is multiplied by itself to 9409.
    Monkey gets bored with item. Worry level is divided by 3 to 3136.
    Current worry level is not divisible by 13.
    Item with worry level 3136 is thrown to monkey 3.
Monkey 3:
  Monkey inspects an item with a worry level of 74.
    Worry level increases by 3 to 77.
    Monkey gets bored with item. Worry level is divided by 3 to 25.
    Current worry level is not divisible by 17.
    Item with worry level 25 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 500.
    Worry level increases by 3 to 503.
    Monkey gets bored with item. Worry level is divided by 3 to 167.
    Current worry level is not divisible by 17.
    Item with worry level 167 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 620.
    Worry level increases by 3 to 623.
    Monkey gets bored with item. Worry level is divided by 3 to 207.
    Current worry level is not divisible by 17.
    Item with worry level 207 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 1200.
    Worry level increases by 3 to 1203.
    Monkey gets bored with item. Worry level is divided by 3 to 401.
    Current worry level is not divisible by 17.
    Item with worry level 401 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 3136.
    Worry level increases by 3 to 3139.
    Monkey gets bored with item. Worry level is divided by 3 to 1046.
    Current worry level is not divisible by 17.
    Item with worry level 1046 is thrown to monkey 1.

After round 1, the monkeys are holding items with these worry levels:

Monkey 0: 20, 23, 27, 26
Monkey 1: 2080, 25, 167, 207, 401, 1046
Monkey 2:
Monkey 3:

Monkeys 2 and 3 aren't holding any items at the end of the round; they both inspected items during the round and threw them all before the round ended.

This process continues for a few more rounds:

After round 2, the monkeys are holding items with these worry levels:
Monkey 0: 695, 10, 71, 135, 350
Monkey 1: 43, 49, 58, 55, 362
Monkey 2:
Monkey 3:

After round 3, the monkeys are holding items with these worry levels:
Monkey 0: 16, 18, 21, 20, 122
Monkey 1: 1468, 22, 150, 286, 739
Monkey 2:
Monkey 3:

After round 4, the monkeys are holding items with these worry levels:
Monkey 0: 491, 9, 52, 97, 248, 34
Monkey 1: 39, 45, 43, 258
Monkey 2:
Monkey 3:

After round 5, the monkeys are holding items with these worry levels:
Monkey 0: 15, 17, 16, 88, 1037
Monkey 1: 20, 110, 205, 524, 72
Monkey 2:
Monkey 3:

After round 6, the monkeys are holding items with these worry levels:
Monkey 0: 8, 70, 176, 26, 34
Monkey 1: 481, 32, 36, 186, 2190
Monkey 2:
Monkey 3:

After round 7, the monkeys are holding items with these worry levels:
Monkey 0: 162, 12, 14, 64, 732, 17
Monkey 1: 148, 372, 55, 72
Monkey 2:
Monkey 3:

After round 8, the monkeys are holding items with these worry levels:
Monkey 0: 51, 126, 20, 26, 136
Monkey 1: 343, 26, 30, 1546, 36
Monkey 2:
Monkey 3:

After round 9, the monkeys are holding items with these worry levels:
Monkey 0: 116, 10, 12, 517, 14
Monkey 1: 108, 267, 43, 55, 288
Monkey 2:
Monkey 3:

After round 10, the monkeys are holding items with these worry levels:
Monkey 0: 91, 16, 20, 98
Monkey 1: 481, 245, 22, 26, 1092, 30
Monkey 2:
Monkey 3:

...

After round 15, the monkeys are holding items with these worry levels:
Monkey 0: 83, 44, 8, 184, 9, 20, 26, 102
Monkey 1: 110, 36
Monkey 2:
Monkey 3:

...

After round 20, the monkeys are holding items with these worry levels:
Monkey 0: 10, 12, 14, 26, 34
Monkey 1: 245, 93, 53, 199, 115
Monkey 2:
Monkey 3:

Chasing all of the monkeys at once is impossible; you're going to have to focus on the two most active monkeys if you want any hope of getting your stuff back. Count the total number of times each monkey inspects items over 20 rounds:

Monkey 0 inspected items 101 times.
Monkey 1 inspected items 95 times.
Monkey 2 inspected items 7 times.
Monkey 3 inspected items 105 times.

In this example, the two most active monkeys inspected items 101 and 105 times. The level of monkey business in this situation can be found by multiplying these together: 10605.

Figure out which monkeys to chase by counting how many items they inspect over 20 rounds. What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
*/
