
trait Display {
    fn display(&self); // As a method
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

    fn display(&self) {
        print!("{}\t{}\t{}\t{:#?}\n", self.name, self.last_name, self.age, self.id);
    }
}

impl Display for Animal {

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

    display(person);
    display_the_2nd(&animal);
}

// There will be as many copies of display() as structs implementing the Display trait
// (Will be creted during compile time)
fn display(to_display: impl Display){
    to_display.display();
}

// Using dynamic dispatch makes the program to select the proper implementatio during runtime
fn display_the_2nd(to_display: &dyn Display){
    to_display.display();
}