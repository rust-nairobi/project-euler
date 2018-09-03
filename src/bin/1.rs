/// Problem 1
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn sum_multiples_3_5(n: i32) -> i32 {
    let sum = (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum();

    sum
}

fn main() {
    println!("the sum of all the multiples of 3 or 5 below 10: {}", sum_multiples_3_5(10));
    println!("the sum of all the multiples of 3 or 5 below 1000: {}", sum_multiples_3_5(1000));
}


#[test]
fn test() {
    assert_eq!(sum_multiples_3_5(10), 23);
    assert_eq!(sum_multiples_3_5(1000), 233168);
}
