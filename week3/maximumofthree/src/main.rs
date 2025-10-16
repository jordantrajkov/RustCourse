fn max_of_three(a: i32, b: i32, c:i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >=c {
        b
    } else {
        c 
    }
}


fn main() {
    let num1 = 10;
    let num2 = 25;
    let num3 = 15;

    let max_number = max_of_three(num1, num2, num3);
    println!("The maximum of {}, {}, and {} is {}", num1, num2, num3, max_number);
}
