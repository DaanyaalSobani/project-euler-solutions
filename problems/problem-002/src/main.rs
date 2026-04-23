use std::env::args;

struct Fib {
    a: u64,
    b: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        return Some(current);
    }
}   

fn fib() -> Fib {
    Fib { a: 0, b: 1 }
}

fn main() {
    let cutoff : u64 = args().nth(1).expect("Please provide a number").parse().expect("Please provide a valid number");

    let result: u64 = fib()
    .take_while(|&n|n <= cutoff)
    .filter(|&n| n % 2 == 0)
    .sum();

    println!("Sum of even Fibonacci numbers up to {}: {}", cutoff, result);
}
