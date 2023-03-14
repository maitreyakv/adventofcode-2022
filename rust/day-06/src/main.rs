use std::fs;
use std::collections::HashSet;

fn find_marker(s: &Vec<char>, packet_size: usize) -> Option<usize> {
    for i in packet_size..s.len() {
        let packet: HashSet<&char> = HashSet::from_iter(&s[i-packet_size..i]);
        if packet.len() == packet_size {
            return Some(i);
        }
    }
    None
}

fn main() {
    let input: Vec<char> = fs::read_to_string("test_input.txt")
                            .unwrap()
                            .chars()
                            .collect();
    
    let result = find_marker(&input, 4).unwrap();
    println!("{}", result);

    let result = find_marker(&input, 14).unwrap();
    println!("{}", result);
}
