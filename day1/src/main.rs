fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("not able to read input file");

    let groups = input.split("\n\n");

    let mut sums: Vec<i32> = groups
        .map(|group| {
            group
                .split("\n")
                .map(|line| line.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();
    sums.sort();

    println!("part 1: {:#?}", sums.last().unwrap());

    // part 2
    sums.reverse();
    let sum_of_three = sums.iter().take(3).fold(0, |sum, calories| sum + calories);
    println!("part 2: {sum_of_three}")
}
