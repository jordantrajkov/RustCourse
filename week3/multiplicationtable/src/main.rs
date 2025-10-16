fn multiplecation_table(n: u32) {
    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n * i);
    }
}



fn main() {
    let number = 1;
    println!("Multiplication table for {}:", number);
    multiplecation_table(number);
    let number = 2;
    println!("Multiplication table for {}:", number);
    multiplecation_table(number);
}
