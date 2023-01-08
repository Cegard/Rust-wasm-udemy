
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

    // Methods
    fn greet(&self) {
        println!("Hello I'm {} {}", self.name, self.last_name);
    }

    fn happy_birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person {
        name: String::from("foo"),
        last_name: "bar".to_string(),
        age: 30
    };

    person.greet();

    println!("\nOld values");
    print!("{}\t{}\t{}\n", person.name, person.last_name, person.age);
    person.happy_birthday();

    let name = person.name;
    let last_name = person.last_name;
    let age = person.age;

    println!("\nNew values");
    print!("{name}\t{last_name}\t{age}\n");
    
    Person::say_hello();
}
 