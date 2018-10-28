// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.

/// Checks if `num` is a palindrome.
/// Increasingly compares beginning and ending digits
fn is_palindrome(num: u64) -> bool {
    let mut number = num;
    let mut leading_div = 1;
    while number > 10 {
        number /= 10;
        leading_div *= 10;
    }
    number = num;
    while number > 10 {
        let ending = number % 10;
        let leading = number / leading_div;
        if ending != leading {
            return false;
        }
        number = (number % leading_div) / 10;
        leading_div /= 100;
    }
    true
}

/// Finds the largest palindrome made from the product of two n-`digit` numbers.
/// This is an extremely naive approach bruteforce method.
/// It finds all possible palindromes and then pick the largest one.
///
/// TODO: Find a more efficient method.
fn problem_4_sol_1(digits: u32) -> Result<u64, &'static str> {
    let a = 10u64.pow(digits - 1)..10u64.pow(digits);
    let mut max_palindrome = 0;
    for a_value in a {
        for b_value in (10u64.pow(digits - 1)..10u64.pow(digits)).rev() {
            let candidate = a_value * b_value;
            if is_palindrome(candidate) && candidate > max_palindrome {
                max_palindrome = candidate;
            }
        }
    }
    if max_palindrome == 0 {
        Err("No palindrome product found")
    } else {
        Ok(max_palindrome)
    }
}

fn main() {
    let digits = 3;
    println!("Largest palindromic product for 2 numbers with {} digits {}",
             digits, problem_4_sol_1(digits).ok().unwrap());
}

#[test]
fn test_is_palindrome() {
    assert_eq!(true, is_palindrome(9009));
    assert_eq!(true, is_palindrome(1234554321));
    assert_eq!(true, is_palindrome(1));
    assert_eq!(false, is_palindrome(12));
}

#[test]
fn test_largest_palindrome() {
    assert_eq!(9009, problem_4_sol_1(2).ok().unwrap());
}
