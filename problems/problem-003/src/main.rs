use std::env::args;

fn is_prime(n: u64) -> bool {
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

fn prime_factors(n: u64) -> Vec<u64> {
    let sqrt_n = (n as f64).sqrt() as u64 + 1;
    let mut factors: Vec<u64> = (2..=sqrt_n)
        .filter(|&x| n % x == 0)
        .filter(|&x| is_prime(x))
        .collect();
    let remaining = factors.iter().fold(n, |acc, &p| {
        let mut a = acc;
        while a % p == 0 { a /= p; }
        a
    });
    if remaining > 1 { factors.push(remaining); }
    factors
}

fn main() {
    let n: u64 = args().nth(1)
    .unwrap_or_else(|| "600851475143".to_string())
    .parse()
    .expect("Please provide a valid number");

    println!("Prime factors of {}: {:#?}", n, prime_factors(n));
}
