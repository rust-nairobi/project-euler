// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 552 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

fn problem_6_sol_1(limit: u32) -> u32 {
    let sum_of_squares: u32 = (1..limit + 1).map(|x| x * x).sum();
    let sum: u32 = (1..limit + 1).sum();
    let square_of_sum = sum.pow(2);
    square_of_sum - sum_of_squares
}

fn main() {
    let limit = 100;
    println!("square_of_sums - sum_of_squares of the first {} numbers is {}",
             limit, problem_6_sol_1(limit));
}

#[test]
fn test_diff_squares_sums() {
    assert_eq!(2640, problem_6_sol_1(10))
}