use utils::Primes;

fn main() {
    let target = 2_000_000;
    let result: u64 = Primes::new().take_while(|&x| x < target).sum();
    println!("Target:{target}, Result: {result}");
}
