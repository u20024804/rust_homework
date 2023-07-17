use crate::cms::{School};

mod cms;

fn main() {
    let mut school: School = School {
        school_id: 0,
        name: "rust school".to_string(),
        grade_count: 0,
        course_count: 0,
        group_count: 0,
    };

    let mut grade1 = school.add_grade("grade1".to_string());
    let mut class1 = grade1.add_class("class1".to_string());
    let mut group1 = school.add_group("group1".to_string());
    let mut course1 = school.add_course("course1".to_string());
    let mut student = class1.add_student("LiLei".to_string());
    student.add_group(&mut group1);
    student.add_course(&mut course1);

    println!("{}", student);
    println!("{:?}", school);
}
