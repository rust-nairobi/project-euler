//145 is an interesting number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//Note: as 1! = 1 and 2! = 2 are not sums they are not included.

#[warn(unused_imports)]
fn factorial_function(num: i32) -> i32 {
    match num {
        0 => 1,
        _ => num * factorial_function(num - 1)
    }
}

fn compute_sum(a: i32, b: i32) -> i32 {
   let mut total = 0;
   for i in a..b {
        let mut sum = 0;
        let x = i.to_string();
        for y in x.chars(){
           let z = (y.to_string()).parse::<i32>().unwrap(); 
           sum += factorial_function(z);
        }

        if i == sum {
           total +=sum;
        }
    }
    return total;
}

fn main() {
    let start: i32 = 0;
    let end: i32 = 1_000_000; 
    let result = compute_sum(start, end); 
    println!("Sum of all numbers : {}", result);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial_function(3), 6);
}

#[test]
fn test_compute_sum(){
    assert_eq!(compute_sum(3, 1000), 145);
}

