// 2520 is the smallest number that can be divided by each of the numbers from
// 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?

/// Computes the gcd of two numbers using the Euclid's algorithm
fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        _ => gcd(b, a % b)
    }
}


/// This is basically computing the lcm of the numbers 1 to 20
///
/// ```lcm(a,b) = a*b/gcd(a,b)```
fn problem_5_sol_1(limit: u64) -> u64 {
    (2..=limit).fold(1, |a, b| a * b / gcd(a, b))
}


fn main() {
    let limit = 20;
    println!("Smallest positive number divisible by all of the numbers from 1 to {}: {}",
             limit, problem_5_sol_1(limit));
}

#[test]
fn test_gcd() {
    assert_eq!(6, gcd(6, 12));
    assert_eq!(6, gcd(12, 6));
    assert_eq!(100, gcd(100, 100));
}

#[test]
fn test_lcm() {
    assert_eq!(2520, problem_5_sol_1(10));
}
