fn main() {

    // Temperature conversion from Fahrenheit to Celsius
    let fahrenheit = 72.0;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F is {}°C", fahrenheit, celsius);

    // Interest calculator

    let principal = 1000.0; 
    let rate = 0.05;
    let years = 3;

    let mut amount = principal;

    for _ in 0 .. years {
        amount += amount * (1.0 + rate);
    }

    let interest = amount - principal;

    println!("Principal: ${:.2}", principal);
    println!("Rate: {:.2}%", rate * 100.0);
    println!("Interest: ${:.2}", interest);

    // Working with Arrays and Tuples 

    let cordinates = (10, 20, 30);
    let (x, y , z) = cordinates;
    let rgb = [255,128,0];

    println!("3D Point: x={}, y={}, z={}",x,y,z);
    println!("RGB Color: r={}, g={}, b={}", rgb[0], rgb[1], rgb[2]);

    // Multiple Variables

    let name = "Alice";
    let age = 30;
    let height = 1.65;  
    let is_student = false;

    println!("Name: {}, Age: {}, Height: {}m, Student: {}", name, age, height, is_student);
}
