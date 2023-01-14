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

struct Animal(String, u32, String); // Struct with no named fields
    
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
    check_person_id(person.id);

    print!("{}\t{}\t{}\t{:#?}\n", empty_person.name, empty_person.last_name, empty_person.age, empty_person.id);
    check_person_id(empty_person.id);

    let dog = Animal("Dog".to_string(), 4, String::from("Woof!"));
    let Animal(animal_type, age, sound) = dog;

    println!("I'm a {} years old {} and I say {}", age, animal_type, sound);
}

fn check_person_id(id: PersonID) {
    
    if let PersonID::IDCard(number) = id {
        println!("ID Card {number}");
    }
    else if let PersonID::Passpport(string_part, number_part) = id {
        println!("Passport number {string_part}-{number_part}");
    }
    // else {
    //     There can be an else here, yet in this case is usless
    //     since the enum PersonID has to be either Passport or IDCard
    // }
}
