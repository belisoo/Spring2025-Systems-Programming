// In class Assignment

// Create a struct Student (major)
struct Student {
    major:String,
}
// Higher order functions update majors

fn update_majors(mut collection: Vec<Student>,behavior:fn(&mut Student,String)) {
// First Order functions, assign_major(student,major_declared)
    for (i, student) in collection.iter_mut().enumerate() {
        let new_major = format!("Major{}", i + 1);
        behavior(student, new_major);
    }
    // Printing to check
    println!("Updated Majors:");
    for (i, student) in collection.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}

fn assign_major(s: &mut Student,major:String){
    s.major = major;

}
// create a vector of students1,2,3 and update all students major
fn main() {
    let students = vec![
        Student { major: String::new() },
        Student { major: String::new() },
        Student { major: String::new() },
    ];

    // Update all student majors using the assign_major function
    update_majors(students, assign_major);
}