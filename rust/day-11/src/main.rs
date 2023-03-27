use std::collections::VecDeque;

type Operation = Box<dyn Fn(usize) -> usize>;
type Test = Box<dyn Fn(usize) -> bool>;

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test
}

impl Monkey {
    fn new(items: Vec<usize>, operation: Operation, test: Test) -> Self {
        Monkey {
            items: VecDeque::from(items),
            operation: operation,
            test: test
        }
    }
}

fn simulate(
    monkeys: &mut [Monkey],
    links_true: &[usize],
    links_false: &[usize],
    rounds: usize
) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            while !monkey.items.is_empty() {
                let mut item = monkey.items.pop_front().unwrap();
                item = (monkey.operation)(item);
                if (monkey.test)(item) {
                    let monkey_receiving = monkeys.get_mut(links_true[i]);
                } else {
                    let monkey_receiving = monkeys.get_mut(links_false[i]);
                }
            }
        }
    }
}

fn main() {
    let mut monkeys = vec![
        Monkey::new(
            vec![83, 88, 96, 79, 86, 88, 70],
            Box::new(|x| x * 5),
            Box::new(|x| x % 11 == 0)
        ),
        Monkey::new(
            vec![59, 63, 98, 85, 68, 72],
            Box::new(|x| x * 11),
            Box::new(|x| x % 5 == 0)
        ),
        Monkey::new(
            vec![90, 79, 97, 52, 90, 94, 71, 70],
            Box::new(|x| x + 2),
            Box::new(|x| x % 19 == 0)
        ),
        Monkey::new(
            vec![97, 55, 62],
            Box::new(|x| x + 5),
            Box::new(|x| x % 13 == 0)
        ),
        Monkey::new(
            vec![74, 54, 94, 76],
            Box::new(|x| x * x),
            Box::new(|x| x % 7 == 0)
        ),
        Monkey::new(
            vec![58],
            Box::new(|x| x + 4),
            Box::new(|x| x % 17 == 0)
        ),
        Monkey::new(
            vec![66, 63],
            Box::new(|x| x + 6),
            Box::new(|x| x % 2 == 0)
        ),
        Monkey::new(
            vec![56, 56, 90, 96, 68],
            Box::new(|x| x + 7),
            Box::new(|x| x % 3 == 0)
        )
    ];

    let links_true = vec![2, 4, 5, 2, 0, 7, 7, 4];
    let links_false = vec![3, 0, 6, 6, 3, 1, 5, 1];

    simulate(&mut monkeys, &links_true, &links_false, 20);
}
