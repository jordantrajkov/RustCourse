fn main() {
    
    let tuple_var: (i32, f64, u8) = (500, 52.2, 3);
    let (x, y, z) = tuple_var; // Destructuring
    println!("The value of x is {} and y is {} and z is {}", x, y, z);

    let sum = x + y + z;
    let avarage = sum / 3;

    println!("The sum is {}", sum);
    println!("The avarage is {}", avarage);
    println!("Product is {}", x * y * z);
}
