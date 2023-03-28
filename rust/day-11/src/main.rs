use std::collections::{VecDeque, HashMap};

type Operation = Box<dyn Fn(usize) -> usize>;
type Test = Box<dyn Fn(usize) -> bool>;

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    link_true: usize,
    link_false: usize,
    num_inspections: usize
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        test: Test,
        link_true: usize,
        link_false: usize
    ) -> Self {
        Monkey {
            items: VecDeque::from(items),
            operation: operation,
            test: test,
            link_true: link_true,
            link_false: link_false,
            num_inspections: 0
        }
    }
}

fn simulate(
    mut monkeys: HashMap<usize, Monkey>,
    rounds: usize,
    new_worry: bool
) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.remove(&i).unwrap();
            while !monkey.items.is_empty() {
                let mut item = monkey.items.pop_front().unwrap();
                item = (monkey.operation)(item);
                monkey.num_inspections += 1;
                item = if new_worry { item % 9699690 } else { item / 3 };
                let link = if (monkey.test)(item) { 
                    monkey.link_true
                } else {
                    monkey.link_false
                };
                let monkey_receiving = monkeys.get_mut(&link).unwrap();
                monkey_receiving.items.push_back(item);
            }
            monkeys.insert(i, monkey);
        }
    }

    let mut num_inspections: Vec<usize> = 
        monkeys.values().map(|m| m.num_inspections).collect();
    num_inspections.sort();
    num_inspections.reverse();

    println!("Monkey business: {}", num_inspections[0] * num_inspections[1])
}

fn setup_monkeys() -> HashMap<usize, Monkey> {
    let monkeys = vec![
        Monkey::new(
            vec![83, 88, 96, 79, 86, 88, 70],
            Box::new(|x| x * 5),
            Box::new(|x| x % 11 == 0),
            2, 3
        ),
        Monkey::new(
            vec![59, 63, 98, 85, 68, 72],
            Box::new(|x| x * 11),
            Box::new(|x| x % 5 == 0),
            4, 0
        ),
        Monkey::new(
            vec![90, 79, 97, 52, 90, 94, 71, 70],
            Box::new(|x| x + 2),
            Box::new(|x| x % 19 == 0),
            5, 6
        ),
        Monkey::new(
            vec![97, 55, 62],
            Box::new(|x| x + 5),
            Box::new(|x| x % 13 == 0),
            2, 6
        ),
        Monkey::new(
            vec![74, 54, 94, 76],
            Box::new(|x| x * x),
            Box::new(|x| x % 7 == 0),
            0, 3
        ),
        Monkey::new(
            vec![58],
            Box::new(|x| x + 4),
            Box::new(|x| x % 17 == 0),
            7, 1
        ),
        Monkey::new(
            vec![66, 63],
            Box::new(|x| x + 6),
            Box::new(|x| x % 2 == 0),
            7, 5
        ),
        Monkey::new(
            vec![56, 56, 90, 96, 68],
            Box::new(|x| x + 7),
            Box::new(|x| x % 3 == 0),
            4, 1
        )
    ];

    let monkeys: HashMap<usize, Monkey> = HashMap::from_iter(
        monkeys.into_iter().enumerate()
    );

    monkeys
}

fn main() {
    let monkeys = setup_monkeys();
    simulate(monkeys, 20, false);

    let monkeys = setup_monkeys();
    simulate(monkeys, 10_000, true);
}
