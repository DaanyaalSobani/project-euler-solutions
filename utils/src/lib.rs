pub fn add(left: u64, right: u64) -> u64 {
    println!("hi from util");
    left + right
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {return false;}
    if n < 4 { return true;}    
    if n % 2 == 0 {return false;}
    let mut i: u64 = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
   }
  true
}

pub struct Primes {
    n: u64,
}
impl Primes {
    pub fn new() -> Self {
        Primes { n: 2 }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.n) {
            self.n += 1;
        }
        let prime = self.n;
        self.n += 1;
        Some(prime)
    }
}