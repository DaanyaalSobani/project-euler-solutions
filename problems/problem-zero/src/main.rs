use std::env::args;

fn good_solution(n: u64) -> u64 {
    (0..=n).filter(|x| x%2!=0).map(|x| x*x).sum()
}
fn better_solution(n: u64) -> u64 {
    let n = (n-1)/2;
    (0..=n).map(|x| (2*x+1)*(2*x+1)).sum()
}
fn best_solution(n: u64) -> u64 {
    let n = (n+1)/2;
    n*(4*n*n-1)/3
}

fn main() {
    let big_n: u64 = args().nth(1).expect("Please provide an argument").parse().expect("Please provide a valid number");
    let result = good_solution(big_n);
    println!("N = {}, Result = {}", big_n, result);
    
    let result = better_solution(big_n);
    println!("N = {}, Result = {}", big_n, result);

    let result = best_solution(big_n);
    println!("N = {}, Result = {}", big_n, result);
}
