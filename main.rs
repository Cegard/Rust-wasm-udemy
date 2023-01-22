// use snake_game_v1::Person;
// use snake_game_v1::Animal;
// use snake_game_v1::display;
// use snake_game_v1::display_the_2nd;

// use snake_game_v1::* imports everything which is public

use snake_game_v1::learn_rust::{Person, Animal, display, display_the_2nd};

fn main() {
    let person = Person::from(
        String::from("foo"),
        "bar".to_string(),
        30,
        snake_game_v1::learn_rust::PersonID::Passpport("XYZ".to_string(), 112233)
    );

    let animal = Animal("Dog".to_string(), 4, "Woof!".to_string());

    display(person);
    display_the_2nd(&animal);
}