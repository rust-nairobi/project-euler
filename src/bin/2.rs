//145 is an interesting number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//Note: as 1! = 1 and 2! = 2 are not sums they are not included.
//Challenges
// 1. Rust is statically typed language. It expects types. With fibonanci 
// the error is eminent when you attempt to multiply with overflow
// 2. Comparing the resulting factorial with the number
// 3. Writing its test
#[warn(unused_imports)]
fn factorial_function(num: i32) -> i32 {
    match num {
        0 => 1,
        _ => num * factorial_function(num - 1)
    }
}

fn main() {
    for i in 10..10000 {
        let mut sum = 0;
        let x = i.to_string();
        for y in x.chars(){
           let z = (y.to_string()).parse::<i32>().unwrap(); 
           sum += factorial_function(z);
        }
        //println!("{}! = {}", i, sum);

        if i == sum {
           println!("An interesting number here sum of {} factorials = {}", i, sum); 
        }
    }
}
