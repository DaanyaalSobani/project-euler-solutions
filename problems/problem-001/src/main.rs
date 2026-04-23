use std::env::args;

fn good_solution(n: u64) -> u64 {
    (1..n).filter(|x| x%3==0 || x%5==0).sum()
}
fn better_solution(n: u64) -> u64 {
    let sum_of_multiples = |k| {
        let p = (n - 1) / k;
        k * p * (p + 1) / 2
    };
    sum_of_multiples(3) + sum_of_multiples(5) - sum_of_multiples(15)
}

fn main() {
    let big_n: u64 = args().nth(1).expect("Please provide an argument").parse().expect("Please provide a valid number");
    let result = good_solution(big_n);
    println!("N = {}, Result = {}", big_n, result);

    let result = better_solution(big_n);
    println!("N = {}, Result = {}", big_n, result);
}
