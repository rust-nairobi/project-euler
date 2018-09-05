/// Problem 1
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, 
/// we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn sum_multiples_3_5(n: i32) -> i32 {
    let sum = (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum();

    sum
}

/// Calculates and returns the sum of multiples of a `num`
/// below the `limit` using arithmetic series
fn sum_multiples(num: i32, limit: i32) -> i32 {
    let num_of_multiples = (limit - 1) / num;
    num * ((num_of_multiples * (num_of_multiples + 1)) / 2)
}

/// Calculates and returns the sum of all the multiples of 3 or 5 below 1000.
/// using the helper function ```sum_multiples```
pub fn problem_1_sol_2(limit: i32) -> i32 {
    let sum_3 = sum_multiples(3, limit);
    let sum_5 = sum_multiples(5, limit);
    let sum_15 = sum_multiples(15, limit);
    sum_3 + sum_5 - sum_15
}

fn main() {
    println!(
        "the sum of all the multiples of 3 or 5 below 10: {}",
        sum_multiples_3_5(10)
    );
    println!(
        "the sum of all the multiples of 3 or 5 below 1000: {}",
        sum_multiples_3_5(1000)
    );
    print!("Using problem_1_sol_2:");
    print!(
        "\tthe sum of all the multiples of 3 or 5 below 1000: {}",
        problem_1_sol_2(1000)
    );
}

#[test]
fn test() {
    assert_eq!(sum_multiples_3_5(10), 23);
    assert_eq!(sum_multiples_3_5(1000), 233168);
}

#[test]
fn sum_of_multiples_3_below_10() {
    assert_eq!(18, sum_multiples(3, 10));
}

#[test]
fn test_sum_below_10() {
    assert_eq!(23, problem_1_sol_2(10));
}
