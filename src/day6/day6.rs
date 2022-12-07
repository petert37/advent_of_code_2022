use std::fs;

fn find_different_characters_of_size(chars: &Vec<char>, size: usize) -> Option<i32> {
    for (i, window) in chars.windows(size).enumerate() {
        if window
            .iter()
            .all(|c| window.iter().filter(|ch| **ch == *c).count() == 1)
        {
            return Some((i + size) as i32);
        }
    }
    return None;
}

fn main() {
    let chars = fs::read_to_string("src/day6/input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    if let Some(first_start_of_packer_number) = find_different_characters_of_size(&chars, 4) {
        println!("Start of packet number: {}", first_start_of_packer_number);
    };

    if let Some(first_start_of_message_number) = find_different_characters_of_size(&chars, 14) {
        println!("Start of message number: {}", first_start_of_message_number);
    };
}
