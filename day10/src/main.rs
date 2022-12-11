use std::convert::From;

#[derive(Debug)]
struct State {
    cycle: u32,
    reg_x: i32,
}

impl State {
    fn new() -> Self {
        Self { cycle: 1, reg_x: 1 }
    }
}

#[derive(Debug)]
enum Cmd {
    NOOP,
    ADDX(i32),
}

#[derive(Debug)]
struct Screen {
    data: [[bool; 40]; 6],
}

impl Screen {
    fn new() -> Self {
        Self {
            data: [[false; 40]; 6],
        }
    }

    fn draw_pixel(&mut self, state: &State) {
        let cursor_x = (state.cycle - 1) % 40;
        let cursor_y = (state.cycle - 1) / 40;
        if cursor_x >= (state.reg_x as u32).clamp(1, u32::MAX) - 1
            && cursor_x <= (state.reg_x as u32).clamp(0, 40) + 1
        {
            self.data[cursor_y as usize][cursor_x as usize] = true;
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            let line = row.map(|pixel| if pixel { "#" } else { "." }).join("");
            if let Err(e) = writeln!(f, "{}", line) {
                return Err(e);
            }
        }
        return Ok(());
    }
}

impl Cmd {
    fn exec(&self, state: &State) -> Vec<State> {
        match self {
            Cmd::NOOP => vec![State {
                cycle: state.cycle + 1,
                reg_x: state.reg_x,
            }],
            Cmd::ADDX(x) => vec![
                State {
                    cycle: state.cycle + 1,
                    reg_x: state.reg_x,
                },
                State {
                    cycle: state.cycle + 2,
                    reg_x: state.reg_x + *x,
                },
            ],
        }
    }
}

fn calculate_signal_strength(states: &Vec<State>) -> i32 {
    let state_score = |state: &State| state.cycle as i32 * state.reg_x;

    let idxs: [usize; 6] = [20, 60, 100, 140, 180, 220];
    return idxs.iter().map(|idx| state_score(&states[*idx - 1])).sum();
}

impl From<&str> for Cmd {
    fn from(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().collect();
        let cmd = words[0];
        match cmd {
            "noop" => Cmd::NOOP,
            "addx" => Cmd::ADDX(words[1].parse::<i32>().unwrap()),
            c => panic!("unknown command {}", c),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let init_state = State::new();
    let cmds: Vec<Cmd> = input.lines().map(|line| Cmd::from(line)).collect();
    let cycle_state = cmds.iter().fold(vec![init_state], |mut states, cmd| {
        states.append(&mut cmd.exec(states.last().unwrap()));
        return states;
    });

    println!("Part 1:{:#?}", calculate_signal_strength(&cycle_state));

    let mut screen = Screen::new();

    cycle_state
        .iter()
        .for_each(|state| screen.draw_pixel(state));
    println!("{}", screen);
}
