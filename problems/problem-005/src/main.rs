// use std::ops::Range;
fn is_divisisable_by_range(n:u64, k: u64)  -> bool
{
    (1..=n).all(|i| k % i == 0)
}

fn main() {
    let n = 20;
    for i in 1u64..{
        if is_divisisable_by_range(n,i){
            println!("{}",i);
            break;
        }
    }
}
