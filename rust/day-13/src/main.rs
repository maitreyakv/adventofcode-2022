use std::{ fmt, fs };
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Clone, Eq, Ord)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>)
}

fn split_packet_string(chars: VecDeque<char>) -> Vec<String> {
    let mut result = Vec::new();

    let mut buf = Vec::new();
    let mut bracket_count = 0;

    for c in chars {
        if c == '[' {
            bracket_count += 1;
        } else if c == ']' {
            bracket_count -= 1;
        }

        if c == ',' && bracket_count == 0 {
            result.push(String::from_iter(buf.drain(..)));

        } else {
            buf.push(c);
        }
    }

    result.push(String::from_iter(buf.drain(..)));

    result
}

impl Packet {
    fn from_string(s: &str) -> Self {
        let mut chars: VecDeque<char> = s.chars().collect();
        if chars[0] == '[' {
            chars.pop_front();
            chars.pop_back();

            let packet = if chars.is_empty() {
                Vec::new()
            } else {
                split_packet_string(chars)
                    .iter()
                    .map(|p| Packet::from_string(p))
                    .collect()
            };

            return Packet::List(packet)
        } else {
            return Packet::Integer(s.parse::<usize>().unwrap())
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => {
                return x == y
            },
            (Packet::List(x), Packet::List(y)) => {
                for (l, r) in zip(x.iter(), y.iter()) {
                    if l != r {
                        return false
                    }
                }
                return x.len() == y.len()
            }
            (Packet::Integer(_), Packet::List(_)) => {
                let x = Packet::List(vec![self.clone()]);
                return x == *other
            },
            (Packet::List(_), Packet::Integer(_)) => {
                let y = Packet::List(vec![other.clone()]);
                return *self == y
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => {
                return x.partial_cmp(y)
            },
            (Packet::List(x), Packet::List(y)) => {
                for (l, r) in zip(x.iter(), y.iter()) {
                    if l < r {
                        return Some(Ordering::Less)
                    } else if l > r {
                        return Some(Ordering::Greater)
                    }
                }
                return x.len().partial_cmp(&y.len())
            }
            (Packet::Integer(_), Packet::List(_)) => {
                let x = Packet::List(vec![self.clone()]);
                return x.partial_cmp(other)
            },
            (Packet::List(_), Packet::Integer(_)) => {
                let y = Packet::List(vec![other.clone()]);
                return self.partial_cmp(&y)
            }
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        match self {
            Packet::Integer(x) => { string.push_str(&x.to_string()); },
            Packet::List(x) => {
                string.push('[');
                for packet in x {
                    string.push_str(&packet.to_string());
                    string.push(',');
                }
                if string.chars().last().unwrap() == ',' {
                    string.pop();
                };
                string.push(']')
            }
        }
        write!(f, "{string}")
    }
}

fn read_packet_pairs() -> Vec<(Packet, Packet)> {
    let file = fs::read_to_string("test_input.txt").unwrap();
    let mut lines = file.lines().peekable();
    let mut packet_pairs = Vec::new();
    while lines.peek().is_some() {
        let packet_1 = Packet::from_string(lines.next().unwrap());
        let packet_2 = Packet::from_string(lines.next().unwrap());
        lines.next();
        packet_pairs.push((packet_1, packet_2))
    }
    packet_pairs
}

fn read_all_packets() -> Vec<Packet> {
    fs::read_to_string("test_input.txt")
        .unwrap()
        .lines()
        .filter(|l| l.len() > 1)
        .map(|l| Packet::from_string(l))
        .collect()
}

fn main() {
    let packet_pairs = read_packet_pairs();
    let result: usize = packet_pairs
                    .iter()
                    .enumerate()
                    .map(|(i, pair)| (i + 1, pair))
                    .filter(|(_, pair)| pair.0 < pair.1)
                    .map(|(i, _)| i)
                    .sum();
    println!("Sum of ordered packet pair indices: {result}");

    let mut packets = read_all_packets();

    let div_1 = Packet::from_string("[[2]]");
    let div_2 = Packet::from_string("[[6]]");

    packets.push(div_1.clone());
    packets.push(div_2.clone());
    packets.sort();

    let result: usize = (0..packets.len())
        .filter(move |&i| packets[i] == div_1 || packets[i] == div_2)
        .map(|i| i + 1)
        .product();

    println!("Product of divider packet indices: {result}");
}
