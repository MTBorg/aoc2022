#[derive(Debug)]
struct Move(usize, usize, usize);

type Stack = Vec<char>;

const WIDTH: u32 = 9;

fn execute_move(mut state: Vec<Stack>, mv: &Move) -> Vec<Stack> {
    let &Move(amount, from, to) = mv;
    let mut taken: Vec<char> = vec![];

    for _ in 0..amount {
        taken.push(state[from].pop().unwrap());
    }

    state[to].append(&mut taken);
    return state;
}

fn execute_move_part_2(mut state: Vec<Stack>, mv: &Move) -> Vec<Stack> {
    let &Move(amount, from, to) = mv;
    let mut taken: Vec<char> = vec![];

    for _ in 0..amount {
        taken.push(state[from].pop().unwrap());
    }
    taken.reverse();

    state[to].append(&mut taken);
    return state;
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let state_lines = input.split("\n\n").collect::<Vec<&str>>()[0];
    let move_lines = input.split("\n\n").collect::<Vec<&str>>()[1];

    let moves: Vec<Move> = move_lines
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|words| Move::from((words[1], words[3], words[5])))
        .collect();

    let initial_state = setup_state(state_lines);
    let final_state = moves
        .iter()
        .fold(initial_state, |state, mv| execute_move(state, &mv));

    let top_crates = final_state
        .iter()
        .map(|stack| stack.last().unwrap())
        .fold(String::new(), |acc, c| format!("{}{}", acc, c));

    println!("Part 1: {top_crates}");

    let initial_state = setup_state(state_lines);
    let final_state = moves
        .iter()
        .fold(initial_state, |state, mv| execute_move_part_2(state, &mv));

    let top_crates = final_state
        .iter()
        .map(|stack| stack.last().unwrap())
        .fold(String::new(), |acc, c| format!("{}{}", acc, c));

    println!("Part 2: {top_crates}");
}

fn setup_state(lines: &str) -> Vec<Stack> {
    let init_lines = lines.lines().rev().skip(1).collect::<Vec<&str>>();

    let mut init_state: Vec<Stack> = vec![];
    for i in 0..WIDTH {
        let idx = i * 4 + 1;
        let mut v: Stack = vec![];
        for line in init_lines.iter() {
            if let Some(c) = line.chars().collect::<Vec<char>>().get(idx as usize) {
                if !c.is_whitespace() {
                    v.push(*c);
                    continue;
                }
            }
            break;
        }
        init_state.push(v);
    }

    return init_state;
}

impl std::convert::From<(&str, &str, &str)> for Move {
    fn from((n, from, to): (&str, &str, &str)) -> Self {
        Self(
            n.parse::<usize>().unwrap(),
            from.parse::<usize>().unwrap() - 1,
            to.parse::<usize>().unwrap() - 1,
        )
    }
}
