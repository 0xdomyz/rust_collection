// rustc supertraits.rs && ./supertraits

trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CompSciStudentImpl {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl CompSciStudent for CompSciStudentImpl {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

impl Person for CompSciStudentImpl {

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Programmer for CompSciStudentImpl {

    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }

}

impl Student for CompSciStudentImpl {
    fn university(&self) -> String {
        self.university.clone()
    }
}

fn main() {
    let student = CompSciStudentImpl {
        name: "John Doe".to_string(),
        university: "MIT".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "johndoe".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}
