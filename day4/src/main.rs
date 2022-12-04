struct Assignment(u32, u32);

fn parse_assignment(assignment: &str) -> Assignment {
    let range: Vec<&str> = assignment.split("-").collect();
    let start = range[0].parse::<u32>().unwrap();
    let end = range[1].parse::<u32>().unwrap();
    return Assignment(start, end);
}

fn parse_line(line: &str) -> (Assignment, Assignment) {
    let assignments: Vec<&str> = line.split(",").collect();
    let a1 = parse_assignment(assignments[0]);
    let a2 = parse_assignment(assignments[1]);
    return (a1, a2);
}

fn fully_overlaps(a1: &Assignment, a2: &Assignment) -> bool {
    let Assignment(s1, e1) = a1;
    let Assignment(s2, e2) = a2;
    (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1)
}

fn any_overlap(a1: &Assignment, a2: &Assignment) -> bool {
    let Assignment(s1, e1) = a1;
    let Assignment(s2, e2) = a2;
    return (s1 >= s2 && s1 <= e2 && e1 >= e2)
        || (e1 >= s2 && e1 <= e2 && s1 <= s2)
        || fully_overlaps(a1, a2);
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let assignment_pairs: Vec<(Assignment, Assignment)> =
        input.lines().map(|line| parse_line(&line)).collect();

    let sum = assignment_pairs
        .iter()
        .filter(|(a1, a2)| fully_overlaps(a1, a2))
        .count();

    println!("part1: {:#?}", sum);

    let sum = assignment_pairs
        .iter()
        .filter(|(a1, a2)| any_overlap(a1, a2))
        .count();

    println!("part2: {:#?}", sum);
}
