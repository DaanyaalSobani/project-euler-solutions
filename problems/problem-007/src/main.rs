use rayon::prelude::*;
use utils::{is_prime, Primes};

fn solution_1(n:usize) {
    let mut m =0;
    for i in 2.. {
        if is_prime(i){
            m += 1;
        }
        if m == n{
            println!("{}",i);
            break;
        }
    }
}

fn solution_2(n: usize) {
    if n == 1 {
        println!("2");
        return;
    }
    let mut m = 1;
    for i in (3u64..).step_by(2) {
        if is_prime(i) {
            m += 1;
        }
        if m == n {
            println!("{}", i);
            break;
        }
    }
}

fn solution_3(n: usize) {
    let nf = n as f64;
    let upper = (nf * nf.ln() * 1.3) as u64 + 100;

    let primes: Vec<u64> = (2..=upper)
        .into_par_iter()
        .filter(|&i| is_prime(i))
        .collect();

    println!("{}", primes[n - 1]);
}

fn main() {
    let n: usize = 10001;
    solution_1(n);
    solution_2(n);
    solution_3(n);

    let (idx, prime) = Primes::new().enumerate().nth(n - 1).unwrap();
    println!("prime #{} = {prime}", idx + 1);
}
