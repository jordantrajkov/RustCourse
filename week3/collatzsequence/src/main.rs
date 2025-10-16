fn collatz_sequence(mut n: u32) {
    println!("Collatz sequence starting from {}:", n);

    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("1");
}



fn main() {
    let starting_number = 6;
    collatz_sequence(starting_number);
    println!("printing multiple sequences:");
    let starting_numbers = [19, 27, 34, 45, 56];
    for &number in &starting_numbers {
        collatz_sequence(number);
    }
}
