use std::fs::File;
use std::io::Write;

fn main() {
    // Create a students vector of tuples 
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting"),
        ("Adams Aliyu", "ECO10110101", "Economics"),
        ("Shania Bolade", "CSC10328828", "Computer"),
        ("Adekunle Gold", "EEE11020202", "Electrical"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical"),
    ];

    let mut file = File::create("students_management.csv").expect("Failed to create");

    // loop through the students vector so the details can be displayed and written to the file 
    for i in 0..students.len() {
        let student_name = students[i].0;
        let student_matric = students[i].1;
        let student_dept = students[i].2;

        println!(
            "Student {}'s name is {}, matric number is {} and belongs to {} department",
            i + 1, student_name, student_matric, student_dept
        );

        // Write the name, matric and department of the particular element to the file 
        file.write_all(student_name.as_bytes()).expect("Failed to write");
        file.write_all(student_matric.as_bytes()).expect("Failed to write");
        file.write_all(student_dept.as_bytes()).expect("Failed to write");
    }
}
