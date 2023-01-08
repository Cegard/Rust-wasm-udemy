
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

    // Method
    fn greet(&self) {
        println!("Hello I'm {} {}", self.name, self.last_name);
    }
}

fn main() {
    let person = Person {
        name: String::from("foo"),
        last_name: "bar".to_string(),
        age: 30
    };

    person.greet();

    let name = person.name;
    let last_name = person.last_name;
    let age = person.age;

    print!("{name}\n{last_name}\n{age}\n");
    Person::say_hello();
}
 