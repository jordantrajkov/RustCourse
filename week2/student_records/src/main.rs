fn main() {

    let student_tuple1 = ("Alice", 20, 3.5);
    let student_tuple2 = ("Bob", 22, 3.8);
    let student_tuple3 = ("Charlie", 19, 3.2);

    println!("Studnet 1:Name {}, Age {}, GPA {}", student_tuple1.0, student_tuple1.1, student_tuple1.2);
    println!("Studnet 2:Name {}, Age {}, GPA {}", student_tuple2.0, student_tuple2.1, student_tuple2.2);
    println!("Studnet 3:Name {}, Age {}, GPA {}", student_tuple3.0, student_tuple3.1, student_tuple3.2);


    let grade = (student_tuple1.2 + student_tuple2.2 + student_tuple3.2) / 3.0;
    println!("Average GPA: {:.2}", grade);


}
