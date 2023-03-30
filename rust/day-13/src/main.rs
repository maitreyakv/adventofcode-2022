use std::collections::VecDeque;
use std::fmt;

enum Packet {
    Integer(usize),
    List(Vec<Packet>)
}

impl Packet {
    fn from_string(s: &str) -> Self {
        let mut chars: VecDeque<char> = s.chars().collect();
        if chars[0] == '[' {
            chars.pop_front();
            chars.pop_back();

            let mut packet = Vec::new();

            // TODO: Implement
            while !chars.is_empty() {
                let c = chars.pop_front();
                dbg!(c);
            }


            return Packet::List(packet)
        } else {
            return Packet::Integer(s.parse::<usize>().unwrap())
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
                string.pop();
                string.push(']')
            }
        }
        write!(f, "{string}")
    }
}

fn main() {
    let x = Packet::from_string("[[1],2,3,[4,5],6]");
    // let x = Packet::List(vec![
    //     Packet::List(vec![
    //         Packet::Integer(1)
    //     ]),
    //     Packet::List(vec![
    //         Packet::Integer(2),
    //         Packet::Integer(3),
    //         Packet::Integer(4)
    //     ])
    // ]);
    println!("{x}");
}
