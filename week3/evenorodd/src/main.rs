fn is_even(n: i32) -> bool {
    n % 2 == 0
}



fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for &number in &numbers {
        if is_even(number) {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }
}
