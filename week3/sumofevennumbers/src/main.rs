fn sum_of_even_numbers(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            sum += i;
            println!("Adding {} to sum, current sum is {}", i, sum);
        }
    }
    sum
}

fn main() {
    let number = 10;
    println!("The sum of even numbers from 1 to {} is {}", number, sum_of_even_numbers(number));
}
