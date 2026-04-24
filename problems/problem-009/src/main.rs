fn euclids_formula(m: u64, n: u64) -> (u64, u64, u64) {
    assert!(m > n && n > 0, "need m > n > 0 (got m={m}, n={n})");
    (m * m - n * n, 2 * m * n, m * m + n * n)
}

fn main() {
    let target = 1000;
    for n in 1..=999 {
        for m in n+1..=1000 {
            let (a, b, c) = euclids_formula(m, n);
            if a+b+c == target {
                println!("Found!");
                println!("{m}, {n}");
                println!("{a}, {b}, {c}");
                println!("{a}x{b}x{c}= {}",a*b*c);
                break;
            }
        }
    }
    // 
}
