#[derive(Debug)]

enum PersonID {
    Passpport(String, u32),
    IDCard(u32),
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
        return Person {
            name: "_".to_string(),
            last_name: String::from("_"),
            age: 1,
            id: PersonID::IDCard(0)
        };
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
        PersonID::Passpport("XYZ".to_string(), 112233)
    );
    
    let empty_person = Person::new_empty();
    
    print!("{}\t{}\t{}\t{:#?}\n", person.name, person.last_name, person.age, person.id);
    print!("{}\t{}\t{}\t{:#?}\n", empty_person.name, empty_person.last_name, empty_person.age, empty_person.id);
    check_person_id(person.id);
    check_person_id(empty_person.id);
}

fn check_person_id(id: PersonID) {
    
    let result = match id {
        PersonID::Passpport(string_part, number_part) => {          //
            format!("Passport number {string_part}-{number_part}")  // This part is called an "arm"
        },                                                          //
        PersonID::IDCard(number) => {
            format!("ID card number {number}")
        }                                                           // Arms have to return the same type among all of them
    };

    println!("{result}");
}
 