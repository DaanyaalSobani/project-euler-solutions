use itertools::Itertools;
fn is_palindrome(n:i32) ->bool{
    let mut n = n;
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n%10);
        n/=10;
    }
    let len = digits.len();
    for i in 0..len/2 {
        if digits[i] != digits[len-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    let result = (100..=999).cartesian_product(100..=999)
        .map(|(x,y)| x*y).filter(|&n| is_palindrome(n)).max().unwrap();
    println!("{:?}",result);
   /*Solution 2 */
   let mut best = (0, 0, 0);
    for x in 100..=999 {
        for y in x..=999 {                  // y >= x avoids duplicate work
            let product = x * y;
            if product > best.2 && is_palindrome(product) {
                best = (x, y, product);
            }
        }
        }
    println!("{} × {} = {}", best.0, best.1, best.2);
}
