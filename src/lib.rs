
pub mod learn_rust {

    mod top_level {
        pub fn say_hi() {
            println!("\n\tHi");
        }

        pub mod bottom_level {
            pub fn say_hello(){
                println!("\tHello\n")
            }
        }
    }

    pub trait Display {
        fn display(&self); // As a method
    }

    #[derive(Debug)]
    pub enum PersonID {
        Passpport(String, u32),
        IDCard(u32),
    }

    pub struct Person{
        // fields //
        name: String,
        last_name: String,
        age: u32,
        id: PersonID
        ///////////
    }

    pub struct Animal(pub String, pub u32, pub String); // Struct with no named fields
        
    impl Person {

        pub fn from(name: String, last_name: String, age: u32, id: PersonID) -> Person {
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

            // Absolute references (crate points to -> src/lib.rs and src/main.rs)
            crate::learn_rust::top_level::say_hi();
            crate::learn_rust::top_level::bottom_level::say_hello();
            //////////////////////////////////////////////////////////////////////

            // Relative references (since they are in the same package?)
            top_level::say_hi();
            top_level::bottom_level::say_hello();
            ////////////////////////////////////////////////////////////
            print!("{}\t{}\t{}\t{:#?}\n", self.name, self.last_name, self.age, self.id);
        }
    }

    impl Display for Animal {

        fn display(&self) {
            print!("{}\t{}\t{}\n", self.0, self.1, self.2);
        }
    }


    // There will be as many copies of display() as structs implementing the Display trait
    // (Will be creted during compile time)
    pub fn display(to_display: impl Display){
        to_display.display();
    }

    // Using dynamic dispatch makes the program to select the proper implementatio during runtime
    pub fn display_the_2nd(to_display: &dyn Display){
        to_display.display();
    }
}