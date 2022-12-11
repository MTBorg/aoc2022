use std::{convert::From, vec};

#[derive(Debug, Clone, PartialEq)]
struct Position(i32, i32); // x, y

impl Position {
    fn mv(&self, mv: &Move) -> Vec<Position> {
        let Move { direction, amount } = mv;
        let (x, y) = (self.0, self.1);
        let mut positions: Vec<Position> = vec![];
        for i in 0..=*amount {
            positions.push(match direction {
                Left => Position(x - i, y),
                Right => Position(x + i, y),
                Up => Position(x, y - i),
                Down => Position(x, y + i),
            });
        }
        return positions;
    }

    fn adjacent(&self, pos: &Position) -> bool {
        let Position(x1, y1) = self;
        let Position(x2, y2) = pos;
        (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::*;

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            c => panic!("unknown direction {}", c),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

fn mv_tail_to_head(tail: &Position, head: &Position) -> Position {
    let Position(tx, ty) = tail;
    let Position(hx, hy) = head;

    if tx == hx && ty == hy || tail.adjacent(head) {
        return Position(*tx, *ty);
    }
    let dx = (hx - tx).signum();
    let dy = (hy - ty).signum();
    return Position(tx + dx, ty + dy);
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|words| Move {
            direction: Direction::from(words[0]),
            amount: words[1].parse::<i32>().unwrap(),
        })
        .collect();

    let visited = perform_moves(&moves, 2);

    println!("Part 1: {:#?}", visited.len());

    let visited = perform_moves(&moves, 10);

    println!("Part 2: {:#?}", visited.len());
}

fn perform_moves(moves: &Vec<Move>, rope_length: usize) -> Vec<Position> {
    let start_position = Position(0, 0);
    let mut knots = vec![start_position; rope_length];
    let mut visited: Vec<Position> = vec![];

    for mv in moves.iter() {
        let positions = knots[0].mv(mv);
        for position in positions.into_iter() {
            knots[0] = position;

            for i in 1..rope_length {
                knots[i] = mv_tail_to_head(&knots[i], &knots[i - 1]);
            }

            let tail = &knots[rope_length - 1];
            if !visited.contains(tail) {
                visited.push(Position(tail.0, tail.1));
            }
        }
    }
    return visited;
}
