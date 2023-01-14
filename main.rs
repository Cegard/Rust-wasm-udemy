
trait Display {
    fn display(&self); // As a method

    fn log(); // As a class function

    // With a default value
    fn show() {
        println!("You forgot to implement this!"); 
    }
}

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

    fn from(name: String, last_name: String, age: u32, id: PersonID) -> Person {
        return Person{
            name,
            last_name,
            age,
            id
        };
    }
}

impl Display for Person {

    fn log() {
        println!("Person struct.");
    }

    fn display(&self) {
        print!("{}\t{}\t{}\t{:#?}\n", self.name, self.last_name, self.age, self.id);
    }
}

impl Display for Animal {

    fn log() {
        println!("Animal struct.");
    }

    fn display(&self) {
        print!("{}\t{}\t{}\n", self.0, self.1, self.2);
    }
}

fn main() {
    let person = Person::from(
        String::from("foo"),
        "bar".to_string(),
        30,
        PersonID::Passpport("XYZ".to_string(), 112233)
    );

    let animal = Animal("Dog".to_string(), 4, "Woof!".to_string());

    Person::log();
    person.display();
    Person::show();

    Animal::log();
    animal.display();
    Animal::show();
}
