use std::fs;

#[derive(Debug)]
struct EncryptedFile {
    data: Vec<isize>,
    original_data: Vec<isize>
}

impl EncryptedFile {
    fn from_vec(v: Vec<isize>) -> Self {
        Self { data: v.clone(), original_data: v }
    }

    fn move_forward(&mut self, x: isize) {
        let i = self.data.iter().position(|&y| x == y).unwrap();
        let n = self.data.len();
        self.data.swap(i, (i + 1) % n);
    }

    fn move_backward(&mut self, x: isize) {
        let i = self.data.iter().position(|&y| x == y).unwrap();
        let n = self.data.len();
        self.data.swap(i, (i - 1 + n) % n);
    }

    fn decrypt(&mut self) -> isize {
        for x in self.original_data.clone() {
            if x > 0 {
                for _ in 0..x {
                    self.move_forward(x);
                }
            } else if x < 0 {
                for _ in 0..-x {
                    self.move_backward(x);
                }
            }
        }

        let i = self.data.iter().position(|&x| x == 0).unwrap();
        let n = self.data.len();
        let coord = vec![1000, 2000, 3000].into_iter()
                                        .map(|di| {
                                            self.data[(i + di) % n]
                                        })
                                        .sum();
        coord
    }
}

fn main() {
    let file = fs::read_to_string("test_input.txt").unwrap();
    let data: Vec<isize> = file.lines()
                               .map(|x| x.parse::<isize>()
                               .unwrap())
                               .collect();

    let mut encrpyted_file = EncryptedFile::from_vec(data);
    let result = encrpyted_file.decrypt();
    println!("decrypted grove coordinates are {result}");
    
}
