fn main() {
    let n = 100;
    let sum_of_squares :u64 = (1..=n).map(|x| x*x).sum();
    let sum_squared: u64 = (1..=n).sum::<u64>().pow(2);
    println!("(1+2...n)^2 - (1^2+2^2...n^2)=");
    println!("n={}",n);
    println!("{} - {} = {}",sum_squared,sum_of_squares,sum_squared-sum_of_squares)
}
