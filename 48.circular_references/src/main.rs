#![allow(dead_code)]

// problem of circular references

// the better solution is to have normalization
// student
// course
// Vec<Enrollment {course, student}>

struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String,
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Self {
        Self { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Self {
        Self {
            enrollments: Vec::new(),
        }
    }
    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

fn main() {
    let john = Student {
        name: "John".into(),
    };
    let course = Course {
        name: "Intro to Rust".into(),
    };

    let mut p = Platform::new();
    p.enroll(&john, &course);

    for c in john.courses(p) {
        println!("John is taking {}", c)
    }
}

// Rc RefCell solution

// student* ---> *course (many to many relation)

// use std::{cell::RefCell, rc::Rc};

// struct Student {
//     name: String,
//     courses: Vec<Rc<RefCell<Course>>>,
// }

// impl Student {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }

// struct Course {
//     name: String,
//     students: Vec<Rc<RefCell<Student>>>,
// }

// impl Course {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }
//     fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student.clone());
//     }
// }

// fn main() {
//     let john = Rc::new(RefCell::new(Student::new("John")));
//     let jane = Rc::new(RefCell::new(Student::new("Jane")));
//     let course = Rc::new(RefCell::new(Course::new("Rust Course")));

//     Course::add_student(course.clone(), john);
//     Course::add_student(course, jane);
// }

// // it is better to avoid cloning
