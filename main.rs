use snake_game_v1::learn_rust::{Person, PersonID};

fn main() {
    let person = Person::from(
        String::from("foo"),
        "bar".to_string(),
        30,
        snake_game_v1::learn_rust::PersonID::Passpport("XYZ".to_string(), 112233)
    );
    let id = PersonID::IDCard(1121);
    println!("{:?}", id);
    println!("{} {}", person.name, person.get_last_name());
}
