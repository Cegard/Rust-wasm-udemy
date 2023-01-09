#[derive(Debug)]

enum PersonID {
    Passpport,
    IDCard,
}

struct Person{
    // fields //
    name: String,
    last_name: String,
    age: u32,
    id: PersonID
    ///////////
}

    
impl Person {

    // Associated fuction, function called from the struct, not from the instaced object
    fn say_hello() {
        println!("Hello");
    }

    // Empty constructor
    fn new_empty() -> Person {
        return Person { name: "_".to_string(), last_name: String::from("_"), age: 1, id: PersonID::IDCard };
    }

    fn new(name: String, last_name: String, age: u32, id: PersonID) -> Person {
        return Person{
            name,
            last_name,
            age,
            id
        };
    }

    // Methods
    fn greet(&self) {
        println!("Hello I'm {} {}", self.name, self.last_name);
    }

    fn happy_birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person::new(
        String::from("foo"),
        "bar".to_string(),
        30,
        PersonID::Passpport
    );
    
    let empty_person = Person::new_empty();
    
    print!("{}\t{}\t{}\t{:#?}\n", person.name, person.last_name, person.age, person.id);
    print!("{}\t{}\t{}\t{:#?}\n", empty_person.name, empty_person.last_name, empty_person.age, empty_person.id);
}
 