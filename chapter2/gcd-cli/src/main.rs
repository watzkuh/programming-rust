use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
        if numbers.len() == 0 {
            eprintln!("Expected at least one argument");
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(m: u64, n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    let remainder = m % n;
    if remainder == 0 {
        return n;
    }
    gcd(n, remainder)
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(4, 16), 4);
}
