use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let mut sum = 0;
    let mut arr = vec![0, 0, 0];
    
    let file = File::open("./test_input.txt".to_string()).unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let l = line.unwrap();
        let l = l.trim();
        let v = l.parse::<u32>();
        match v {
            Ok(n) => {
                sum += n;
            },
            Err(_) => {
                arr.push(sum);
                arr.sort();
                arr.reverse();
                arr.pop();
                sum = 0;
            }
        }
    }

    println!("{}", arr[0]);

    let result: u32 = arr.iter().sum(); 
    println!("{result}");
}