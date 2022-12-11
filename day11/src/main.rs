use itertools::Itertools;
use std::convert::From;

struct Operation(Box<dyn Fn(u64) -> u64>);
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Op")
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: Test,
    inspected_items: u64,
}

impl Monkey {
    fn inspect_items(&mut self, max_size: u64) {
        self.inspected_items += self.items.len() as u64;
        let Operation(op) = &self.op;

        // Part 1
        // self.items.iter_mut().for_each(|item| {
        //     *item = op(*item) / 3;
        // });

        // Part 2
        self.items.iter_mut().for_each(|item| {
            *item = op(*item) % max_size;
        });
    }

    fn get_throws(&mut self) -> Vec<(usize, u64)> {
        self.items
            .drain(0..)
            .map(|item| {
                let target = if item % self.test.divisible_by == 0 {
                    self.test.target_true
                } else {
                    self.test.target_false
                };
                return (target, item);
            })
            .collect()
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();
        let op = Operation::from(lines[2]);
        let test = Test::from(&lines[3..=5]);
        let items: Vec<_> = lines[1]
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|split| split.trim().parse::<u64>().unwrap())
            .collect();

        Self {
            items,
            op,
            test,
            inspected_items: 0,
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let words: Vec<_> = s.split_whitespace().collect();
        let op = words[words.len() - 2];
        let val = words[words.len() - 1];
        let op_val = if val == "old" {
            None
        } else {
            Some(val.parse().unwrap())
        };
        let get_op_val = move |old: u64| op_val.unwrap_or(old);
        match op {
            "+" => Operation(Box::new(move |old| old + get_op_val(old))),
            "*" => Operation(Box::new(move |old| old * get_op_val(old))),
            o => panic!("unknown operation {}", o),
        }
    }
}

fn play_round(monkeys: &mut Vec<Monkey>) {
    let max_size: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    for i in 0..monkeys.len() {
        monkeys[i].inspect_items(max_size);
        let throws = monkeys[i].get_throws();
        throws
            .into_iter()
            .for_each(|(target, item)| monkeys[target].items.push(item));
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    target_true: usize,
    target_false: usize,
}

impl From<&[&str]> for Test {
    fn from(s: &[&str]) -> Self {
        let words: Vec<_> = s[0].split_whitespace().collect();
        let divisible_by = words[words.len() - 1].parse::<u64>().unwrap();

        let words: Vec<_> = s[1].split_whitespace().collect();
        let target_true = words[words.len() - 1].parse::<usize>().unwrap();

        let words: Vec<_> = s[2].split_whitespace().collect();
        let target_false = words[words.len() - 1].parse::<usize>().unwrap();

        Self {
            divisible_by,
            target_true,
            target_false,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let monkey_business = |monkeys: &Vec<Monkey>| {
        monkeys
            .iter()
            .map(|monkey| monkey.inspected_items)
            .sorted_by(|i, j| j.cmp(i))
            .take(2)
            .product::<u64>()
    };

    // Part 1
    {
        let mut monkeys: Vec<_> = input
            .split("\n\n")
            .map(|split| Monkey::from(split))
            .collect();

        (0..20).into_iter().for_each(|_| play_round(&mut monkeys));

        println!("Part 1: {}", monkey_business(&monkeys));
    }

    // Part 2
    {
        let mut monkeys: Vec<_> = input
            .split("\n\n")
            .map(|split| Monkey::from(split))
            .collect();

        (0..10_000)
            .into_iter()
            .for_each(|_| play_round(&mut monkeys));

        println!("Part 2: {}", monkey_business(&monkeys));
    }
}
