use std::fs;

struct Device {
    x: isize,
    cycle: usize,
    signal_strengths: Vec<isize>,
    screen: String
}

impl Device {
    fn new() -> Self {
        Device {
            x: 1,
            cycle: 1,
            signal_strengths: Vec::new(), 
            screen: "".to_string()
        }
    }

    fn perform_cycle(&mut self) {
        if (self.cycle + 20) % 40 == 0 {
            self.signal_strengths.push(self.x * self.cycle as isize);
        }
        let position = (self.cycle - 1) as isize;
        let pixel = if position % 40 >= self.x - 1 && position % 40 <= self.x + 1 {
            '#'
        } else {
            '.'
        };
        self.screen.push(pixel);
        self.cycle += 1;
    }

    fn execute(&mut self, instruction: &str) {
        if instruction.contains("noop") {
            self.perform_cycle()
        } else if instruction.contains("addx") {
            self.perform_cycle();
            self.perform_cycle();
            let (_, value) = instruction.split_once(' ').unwrap();
            self.x += value.parse::<isize>().unwrap();
        } else {
            panic!("Unrecognized instruction!")
        }
    }

    fn print_screen(&self) {
        for (i, pixel) in self.screen.chars().enumerate() {
            print!("{}", pixel);
            if (i + 1) % 40 == 0 {
                println!("")
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("test_input.txt").unwrap();

    let mut device = Device::new();

    for line in file.lines() {
        device.execute(&line);
    }

    let result = device.signal_strengths.iter().sum::<isize>();
    println!("Sum of signal strengths: {}", result);

    device.print_screen();
}
