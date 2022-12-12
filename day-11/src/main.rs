use common::*;
use std::collections::VecDeque;

/*
Each monkey has several attributes:

- Starting items lists your worry level for each item the monkey is currently holding in the order they will be inspected.
- Operation shows how your worry level changes as that monkey inspects an item. (An operation like new = old * 5 means that your worry level after the monkey inspected the item is five times whatever your worry level was before inspection.)
- Test shows how the monkey uses your worry level to decide where to throw an item next.
  -   If true shows what happens with an item if the Test was true.
  -   If false shows what happens with an item if the Test was false.
*/

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    let result2 = part2(input);

    present_result(result1, Some(result2));
}

// 20 rounds of monkey-ing with worry management of 'worry / 3' after inspection
fn part1(input: &str) -> u64 {
    let mut monkeys = get_monkey_vec_from_input(input);
    for _i in 0..20 {
        for i in 0..monkeys.len() {
            let (monkey, throws) = monkeys[i].clone().conduct_buisness(None);
            monkeys[i] = monkey;
            for throw in throws {
                monkeys[throw.to].held_items.push_back(throw.item);
            }
        }
    }

    monkeys.sort_by(|monkey_a, monkey_b| monkey_b.inspect_num.cmp(&monkey_a.inspect_num));
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspect_num)
        .product::<u64>()
}

// 10_000 rounds of monkey-ing with manual worry management
fn part2(input: &str) -> u64 {
    let mut monkeys = get_monkey_vec_from_input(input);
    let common_dividend = monkeys.iter().map(|monkey| monkey.divisor).product::<u64>();
    for _i in 0..10_000 {
        for i in 0..monkeys.len() {
            let (monkey, throws) = monkeys[i].clone().conduct_buisness(Some(common_dividend));
            monkeys[i] = monkey;
            for throw in throws {
                monkeys[throw.to].held_items.push_back(throw.item);
            }
        }
    }

    monkeys.sort_by(|monkey_a, monkey_b| monkey_b.inspect_num.cmp(&monkey_a.inspect_num));
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspect_num)
        .product::<u64>()
}

fn get_monkey_vec_from_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|seed| {
            seed.split('\n')
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .map(Monkey::new)
        .collect::<Vec<Monkey>>()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Old,
    Const(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    held_items: VecDeque<u64>,
    operator: Operator,
    operand: Operand,
    divisor: u64,
    true_friend: usize,
    false_friend: usize,
    inspect_num: u64,
}

struct Throw {
    item: u64,
    to: usize,
}

impl Monkey {
    fn new(seed: Vec<String>) -> Self {
        use Operand::*;
        use Operator::*;

        let mut held_items = VecDeque::new();
        let (_, items) = seed[1].split_at(18);
        held_items.extend(items.split(", ").map(str::parse::<u64>).map(Result::unwrap));

        let operator_context = seed[2]
            .split_at(23)
            .1
            .split(' ')
            .map(str::to_string)
            .collect::<Vec<String>>();
        let operator = match operator_context[0].as_str() {
            "+" => Add,
            "*" => Mul,
            _ => panic!("Unexpected Operator"),
        };
        let operand = match operator_context[1].as_str() {
            "old" => Old,
            var => Const(var.parse::<u64>().unwrap()),
        };

        let divisor = seed[3].split_at(21).1.parse::<u64>().unwrap();
        let true_friend = seed[4].split_at(29).1.parse::<usize>().unwrap();
        let false_friend = seed[5].split_at(30).1.parse::<usize>().unwrap();

        Self {
            held_items,
            operator,
            operand,
            divisor,
            true_friend,
            false_friend,
            inspect_num: 0,
        }
    }

    fn conduct_buisness(mut self, manual_worry_management: Option<u64>) -> (Self, Vec<Throw>) {
        let mut throws = Vec::new();
        while let Some(mut item) = self.held_items.pop_front() {
            self.inspect_num += 1;
            if let Some(divider) = manual_worry_management {
                item = self.cause_concern(item);
                item = item.rem_euclid(divider);
            } else {
                item = self.cause_concern(item) / 3;
            }
            self.throw(&mut throws, item, item % self.divisor == 0)
        }
        (self, throws)
    }

    fn throw(&self, throws: &mut Vec<Throw>, item: u64, condition: bool) {
        if condition {
            throws.push(Throw {
                item,
                to: self.true_friend,
            });
        } else {
            throws.push(Throw {
                item,
                to: self.false_friend,
            });
        }
    }

    fn cause_concern(&self, item: u64) -> u64 {
        use Operand::*;
        use Operator::*;

        match (self.operator, self.operand) {
            (Add, Old) => item + item,
            (Mul, Old) => item * item,
            (Add, Const(i)) => item + i,
            (Mul, Const(i)) => item * i,
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(TEST_INPUT), 10605);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT), 2_713_310_158);
    }
}
