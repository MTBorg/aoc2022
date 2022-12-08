fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let chars = input.chars().collect::<Vec<char>>();
    let start_of_packet = search_for_unique_slice(&chars, 4);
    println!("Part 1: {start_of_packet}");

    let start_of_message = search_for_unique_slice(&chars, 14);
    println!("Part 1: {start_of_message}");
}

fn search_for_unique_slice(chars: &Vec<char>, slice_size: usize) -> usize {
    for i in 0..chars.len() - slice_size {
        if slice_is_unique(&chars[i..i + slice_size]) {
            return i + slice_size;
        }
    }
    panic!("no unique slice found")
}

fn slice_is_unique(slice: &[char]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}
