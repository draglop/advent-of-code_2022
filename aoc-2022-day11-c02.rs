// https://adventofcode.com/2022/day/11
// (part 2)

const MONKEY_LINE_ID_PREFIX: &str = "Monkey ";
const MONKEY_LINE_ITEMS_PREFIX: &str = "  Starting items: ";
const MONKEY_LINE_OPERATION_PREFIX: &str = "  Operation: new = old ";
const MONKEY_LINE_TEST_PREFIX: &str = "  Test: divisible by ";
const MONKEY_LINE_IFFALSE_PREFIX: &str = "    If false: throw to monkey ";
const MONKEY_LINE_IFTRUE_PREFIX: &str = "    If true: throw to monkey ";
const ROUNDS_COUNT: usize = 10000;

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

fn find_cm(monkeys: &Vec<Monkey>) -> usize {
    let mut cm = 1;

    for monkey in monkeys {
        cm *= monkey.divisible_by;
    }

    return cm;
}

fn update_worry_level(mut worry_level: usize, operation: MonkeyOperation, cm: usize) -> usize {
    match operation.operator {
        Operator::ADD => worry_level += operation.operand,
        Operator::MULTIPLY => worry_level *= operation.operand,
        Operator::SQUARE => worry_level *= worry_level,
        _ => panic!("expected MonkeyOperation"),
    }

    worry_level %= cm;

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

    let cm: usize = find_cm(&monkeys);

    let mut inspected_counts: Vec<usize> = Vec::with_capacity(monkeys.len());
    inspected_counts.resize(monkeys.len(), 0);

    for _ in 0..ROUNDS_COUNT {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone(); // pfff !
            for item in &monkey.items {
                let worry_level = update_worry_level(*item, monkey.operation, cm);
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
    assert_eq!(2713310158, monkey_business_level(
"Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1"));
    // user puzzle
    assert_eq!(30616425600, monkey_business_level("Monkey 0:\n  Starting items: 63, 84, 80, 83, 84, 53, 88, 72\n  Operation: new = old * 11\n  Test: divisible by 13\n    If true: throw to monkey 4\n    If false: throw to monkey 7\n\nMonkey 1:\n  Starting items: 67, 56, 92, 88, 84\n  Operation: new = old + 4\n  Test: divisible by 11\n    If true: throw to monkey 5\n    If false: throw to monkey 3\n\nMonkey 2:\n  Starting items: 52\n  Operation: new = old * old\n  Test: divisible by 2\n    If true: throw to monkey 3\n    If false: throw to monkey 1\n\nMonkey 3:\n  Starting items: 59, 53, 60, 92, 69, 72\n  Operation: new = old + 2\n  Test: divisible by 5\n    If true: throw to monkey 5\n    If false: throw to monkey 6\n\nMonkey 4:\n  Starting items: 61, 52, 55, 61\n  Operation: new = old + 3\n  Test: divisible by 7\n    If true: throw to monkey 7\n    If false: throw to monkey 2\n\nMonkey 5:\n  Starting items: 79, 53\n  Operation: new = old + 1\n  Test: divisible by 3\n    If true: throw to monkey 0\n    If false: throw to monkey 6\n\nMonkey 6:\n  Starting items: 59, 86, 67, 95, 92, 77, 91\n  Operation: new = old + 5\n  Test: divisible by 19\n    If true: throw to monkey 4\n    If false: throw to monkey 0\n\nMonkey 7:\n  Starting items: 58, 83, 89\n  Operation: new = old * 19\n  Test: divisible by 17\n    If true: throw to monkey 2\n    If false: throw to monkey 1"));
}

/*
--- Part Two ---

You're worried you might not ever get your items back. So worried, in fact, that your relief that a monkey's inspection didn't damage an item no longer causes your worry level to be divided by three.

Unfortunately, that relief was all that was keeping your worry levels from reaching ridiculous levels. You'll need to find another way to keep your worry levels manageable.

At this rate, you might be putting up with these monkeys for a very long time - possibly 10000 rounds!

With these new rules, you can still figure out the monkey business after 10000 rounds. Using the same example above:

== After round 1 ==
Monkey 0 inspected items 2 times.
Monkey 1 inspected items 4 times.
Monkey 2 inspected items 3 times.
Monkey 3 inspected items 6 times.

== After round 20 ==
Monkey 0 inspected items 99 times.
Monkey 1 inspected items 97 times.
Monkey 2 inspected items 8 times.
Monkey 3 inspected items 103 times.

== After round 1000 ==
Monkey 0 inspected items 5204 times.
Monkey 1 inspected items 4792 times.
Monkey 2 inspected items 199 times.
Monkey 3 inspected items 5192 times.

== After round 2000 ==
Monkey 0 inspected items 10419 times.
Monkey 1 inspected items 9577 times.
Monkey 2 inspected items 392 times.
Monkey 3 inspected items 10391 times.

== After round 3000 ==
Monkey 0 inspected items 15638 times.
Monkey 1 inspected items 14358 times.
Monkey 2 inspected items 587 times.
Monkey 3 inspected items 15593 times.

== After round 4000 ==
Monkey 0 inspected items 20858 times.
Monkey 1 inspected items 19138 times.
Monkey 2 inspected items 780 times.
Monkey 3 inspected items 20797 times.

== After round 5000 ==
Monkey 0 inspected items 26075 times.
Monkey 1 inspected items 23921 times.
Monkey 2 inspected items 974 times.
Monkey 3 inspected items 26000 times.

== After round 6000 ==
Monkey 0 inspected items 31294 times.
Monkey 1 inspected items 28702 times.
Monkey 2 inspected items 1165 times.
Monkey 3 inspected items 31204 times.

== After round 7000 ==
Monkey 0 inspected items 36508 times.
Monkey 1 inspected items 33488 times.
Monkey 2 inspected items 1360 times.
Monkey 3 inspected items 36400 times.

== After round 8000 ==
Monkey 0 inspected items 41728 times.
Monkey 1 inspected items 38268 times.
Monkey 2 inspected items 1553 times.
Monkey 3 inspected items 41606 times.

== After round 9000 ==
Monkey 0 inspected items 46945 times.
Monkey 1 inspected items 43051 times.
Monkey 2 inspected items 1746 times.
Monkey 3 inspected items 46807 times.

== After round 10000 ==
Monkey 0 inspected items 52166 times.
Monkey 1 inspected items 47830 times.
Monkey 2 inspected items 1938 times.
Monkey 3 inspected items 52013 times.

After 10000 rounds, the two most active monkeys inspected items 52166 and 52013 times. Multiplying these together, the level of monkey business in this situation is now 2713310158.

Worry levels are no longer divided by three after each item is inspected; you'll need to find another way to keep your worry levels manageable. Starting again from the initial state in your puzzle input, what is the level of monkey business after 10000 rounds?
*/
