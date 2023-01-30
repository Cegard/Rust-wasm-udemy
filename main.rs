use snake_game_v1::top_mod::learn_rust::{Person};

fn main() {
    let person = Person::from(
        String::from("foo"),
        "bar".to_string(),
        30,
        snake_game_v1::top_mod::learn_rust::PersonID::Passpport("XYZ".to_string(), 112233)
    );
    
    println!("{}", person.id);
}
