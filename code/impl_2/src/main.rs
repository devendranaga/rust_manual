struct Name {
    person : String,
}

impl Name {
    fn set(&mut self, name : String) {
        self.person = name;
    }

    fn print(&self) { println!("name: {}", self.person); }
}

fn main() {
    let mut n = Name {
        person : "Dev".to_string(),
    };
    n.set("dev".to_string());
    n.print();
}
