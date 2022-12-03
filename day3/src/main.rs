#[derive(Debug, Clone)]
struct Compartment(Vec<char>);

fn char_score(c: &char) -> u32 {
    match c {
        'a'..='z' => 1 + (*c as u32 - 'a' as u32),
        'A'..='Z' => 27 + (*c as u32 - 'A' as u32),
        _ => panic!("Unrecognized character"),
    }
}

fn exists_in_all_lines(c: &char, lines: &Vec<Vec<char>>) -> bool {
    lines.iter().filter(|line| line.contains(c)).count() == 3
}

fn find_common_char(group: &Vec<Vec<char>>) -> char {
    ('a'..='z')
        .chain('A'..='Z')
        .find(|c| exists_in_all_lines(&c, &group))
        .unwrap()
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let rucksacks: Vec<(Compartment, Compartment)> = input
        .lines()
        .map(|line| {
            (
                Compartment(line.chars().take(line.len() / 2).collect()),
                Compartment(
                    line.chars()
                        .skip(line.len() / 2)
                        .take(line.len() / 2)
                        .collect(),
                ),
            )
        })
        .collect();

    let errors: Vec<char> = rucksacks
        .iter()
        .map(|(Compartment(c1), Compartment(c2))| {
            *c1.iter()
                .find(|item| c2.iter().find(|item2| item == item2).is_some())
                .unwrap()
        })
        .collect();

    let sum: u32 = errors.iter().map(|c| char_score(c)).sum();
    println!("part1: {sum}");

    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let groups: Vec<Vec<Vec<char>>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .as_slice()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let errors = groups
        .iter()
        .map(|badge| find_common_char(badge))
        .collect::<Vec<char>>();

    let sum: u32 = errors.iter().map(|error| char_score(&error)).sum();
    println!("part2: {sum}");
}
