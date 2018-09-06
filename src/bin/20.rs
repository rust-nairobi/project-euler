// n! means n × (n − 1) × ... × 3 × 2 × 1
//
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//
// Find the sum of the digits in the number 100!

extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::FromPrimitive;


/// Computes the Factorial the BigUint type
fn factorial(num: u32) -> BigUint {
    (1..=num).map(BigUint::from).product()
}

fn problem_20_sol_1(limit: u32) -> u32 {
    let sum_str = format!("{}", factorial(limit));
    sum_str.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn main() {
    let limit = 100;
    println!("Sum of the digits in the number {}!: {}",
             limit, problem_20_sol_1(100));
}

#[test]
fn test_factorial() {
    assert_eq!(BigUint::from_u32(3628800).unwrap(), factorial(10));
}

#[test]
fn test_problem_20_sol_1() {
    assert_eq!(27, problem_20_sol_1(10));
}