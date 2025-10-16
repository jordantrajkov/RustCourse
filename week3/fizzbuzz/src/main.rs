fn fizzbuzz(n : u32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}


fn main() {
    let number = 100;
    println!("FizzBuzz up to {}:", number);
    fizzbuzz(number);
}
