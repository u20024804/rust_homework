use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct School {
    pub(crate) school_id: i32,
    pub(crate) name: String,
    pub(crate) grade_count: i32,
    pub(crate) course_count: i32,
    pub(crate) group_count: i32,
}

#[derive(Debug)]
pub struct Grade {
    grade_id: i32,
    name: String,
    class_count: i32,
}

#[derive(Debug)]
pub struct Class<'a> {
    class_id: i32,
    grade: &'a mut Grade,
    name: String,
    student_count: i32,
}

#[derive(Debug)]
pub struct Course {
    course_id: i32,
    name: String,
}

#[derive(Debug)]
pub struct Group {
    group_id: i32,
    name: String,
}

#[derive(Debug)]
pub struct Student<'b> {
    student_id: i32,
    name: String,
    class: &'b mut Class<'b>,
    courses: HashMap<i32, &'b Course>,
    groups: HashMap<i32, &'b Group>,
}

impl<'a> Student<'a> {
    pub fn new(student_id: i32, name: String, class: &'a mut Class<'a>) -> Self {
        Student {
            student_id,
            name,
            class,
            courses: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn add_course(&mut self, course: &'a Course) {
        self.courses.insert(course.course_id,course);
    }

    pub fn add_group(&mut self, group: &'a Group) {
        self.groups.insert(group.group_id, group);
    }

    pub fn remove_course(&mut self, course: &Course) {
        self.courses.remove(&course.course_id);
    }

    pub fn remove_remove(&mut self, group: &Group) {
        self.groups.remove(&group.group_id);
    }
}

impl School {
    pub fn add_grade(&mut self, name: String) -> Grade {
        let grade: Grade = Grade {
            grade_id: self.grade_count,
            name,
            class_count: 0,
        };
        self.grade_count += 1;
        grade
    }

    pub fn add_course(&mut self, name: String) -> Course {
        let course: Course = Course {
            course_id: self.course_count,
            name,
        };
        self.course_count += 1;
        course
    }

    pub fn add_group(&mut self, name: String) -> Group {
        let group: Group = Group {
            group_id: self.group_count,
            name,
        };
        self.group_count += 1;
        group
    }
}

impl Grade {
    pub fn add_class(&mut self, name: String) -> Class {
        self.class_count += 1;

        let class: Class = Class {
            class_id: self.class_count - 1,
            grade: self,
            name,
            student_count: 0
        };
        class
    }
}

impl<'a> Class<'a> {
    pub fn add_student<'b: 'a>(& 'b mut self, name: String) -> Student<'b> {
        self.student_count += 1;
        Student::new(self.student_count - 1, name, self)
    }
}

impl<'a> fmt::Display for Student<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}