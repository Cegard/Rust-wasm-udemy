
struct Person{
    // fields //
    name: String,
    last_name: String,
    age: u32,
    ///////////
}

    
impl Person {

    // Associated fuction, function called from the struct, not from the instaced object
    fn say_hello() {
        println!("Hello");
    }

    // Empty constructor
    fn new_empty() -> Person {
        return Person { name: "_".to_string(), last_name: String::from("_"), age: 1 };
    }

    fn new(name: String, last_name: String, age: u32) -> Person {
        return Person{
            name,
            last_name,
            age
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
        30
    );
    
    let empty_person = Person::new_empty();
    
    Person::say_hello();

    println!("\nOld values");
    print!("{}\t{}\t{}\n", person.name, person.last_name, person.age);

    person.greet();
    person.happy_birthday();

    let name = person.name;
    let last_name = person.last_name;
    let age = person.age;

    println!("\nNew values");
    print!("{name}\t{last_name}\t{age}\n");

    println!("\nFrom empty constructor");
    print!("{}\t{}\t{}\n", empty_person.name, empty_person.last_name, empty_person.age);
}
 