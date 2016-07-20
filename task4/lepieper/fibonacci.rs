use std::iter::Iterator;

fn main() {
    for i in Fibo::new().take(20) {
        println!("{}", i);
    }
}

struct Fibo {
    current: u32,
    previous: u32,
}

impl Iterator for Fibo {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.previous;
        self.previous = self.current;
        self.current = temp + self.previous;
        Some(temp)
    }
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo {
            current: 1,
            previous: 0,
        }
    }
}
