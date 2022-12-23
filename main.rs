
struct Person{
    name: String,
    last_name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("foo"),
        last_name: "bar".to_string(),
        age: 30
    };

    let name = person.name;
    let last_name = person.last_name;
    let age = person.age;

    print!("{name}\n{last_name}\n{age}\n");
}
 