use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

impl Operation {
    fn perform(&self, left: usize, right: usize) -> usize {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right
        }
    }
}

#[derive(Debug)]
enum Job {
    Number(usize),
    Math(String, String, Operation)
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job
}

impl Monkey {
    fn from_string(line: &str) -> Monkey {
        let (name, mut job) = line.split_once(':').unwrap();
        job = job.trim();

        let job = if job.contains("+") {
            let (left, right) = job.split_once('+').unwrap();
            Job::Math(
                left.trim().to_string(),
                right.trim().to_string(),
                Operation::Add
            )
        } else if job.contains("-") {
            let (left, right) = job.split_once('-').unwrap();
            Job::Math(
                left.trim().to_string(),
                right.trim().to_string(),
                Operation::Sub
            )
        } else if job.contains("*") {
            let (left, right) = job.split_once('*').unwrap();
            Job::Math(
                left.trim().to_string(),
                right.trim().to_string(),
                Operation::Mul
            )
        } else if job.contains("/") {
            let (left, right) = job.split_once('/').unwrap();
            Job::Math(
                left.trim().to_string(),
                right.trim().to_string(),
                Operation::Div
            )
        } else {
            Job::Number(job.parse().unwrap())
        };
    
        Monkey {
            name: name.to_string(),
            job: job
        }
    }
}

fn read_input() -> HashMap<String, Monkey> {
    let contents = fs::read_to_string("test_input.txt").unwrap();
    contents.lines()
            .map(|s: &str| {
                let monkey = Monkey::from_string(s);
                (monkey.name.clone(), monkey)
            })
            .collect()
}

fn solve(monkeys: HashMap<String, Monkey>) -> usize {
    let mut yelled: HashMap<String, usize> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    stack.push("root".to_string());

    while !stack.is_empty() {
        let next_name = stack.pop().unwrap();
        let next = monkeys.get(&next_name).unwrap();

        match &next.job {
            Job::Number(number) => {
                yelled.insert(next_name, *number);
            },
            Job::Math(left, right, op) => {
                match (yelled.get(left), yelled.get(right)) {
                    (Some(yell_left), Some(yell_right)) => {
                        let next_yell = op.perform(*yell_left, *yell_right);
                        yelled.insert(next_name, next_yell);
                    },
                    _ => {
                        stack.push(next_name);
                        stack.push(left.to_string());
                        stack.push(right.to_string());
                    }
                }
            }
        }
    }

    *yelled.get("root").unwrap()
}

fn main() {
    let monkeys = read_input();
    let result = solve(monkeys);
    println!("root monkey yells out {result}");
}
