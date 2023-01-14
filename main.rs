use snake_game::Person;
use snake_game::Animal;
use snake_game::display;
use snake_game::display_the_2nd;

fn main() {
    let person = Person::from(
        String::from("foo"),
        "bar".to_string(),
        30,
        snake_game::PersonID::Passpport("XYZ".to_string(), 112233)
    );

    let animal = Animal("Dog".to_string(), 4, "Woof!".to_string());

    display(person);
    display_the_2nd(&animal);
}