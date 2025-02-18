struct Student {
    name: String,
    noma: u64,
    age: u8,
}

impl Student {
    fn print_student (&self) -> () {
        println!("Name: {}, Noma: {}, Age: {}", self.name, self.noma, self.age);
    }

    fn rename (&mut self, new_name: String) -> () {
        self.name = new_name;
    }
}

fn main () {
    let mut student = Student {
        name: String::from("Matteo"),
        noma: 11772100,
        age: 21
    };

    student.print_student();

    student.rename(String::from("Racteur"));

    student.print_student();
}