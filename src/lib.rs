
// The mod keyword here imports the modules in the lib file
mod lib_two;

// The use keyword here allows a less verbose access 
// use lib_two::mod_two;

fn outsider() {
    println!("From the outsider function");
}

pub mod top_mod {
    pub mod learn_rust {
        use crate::lib_two::mod_two;


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
            pub name: String,
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

            pub fn get_last_name(&self) -> &String {
                return &self.last_name;
            }
        }

        impl Display for Person {

            fn display(&self) {
                // crate::outsider();

                // The "super" keyword moves to one module up (like writing ".." on directories)
                super::super::outsider();
                mod_two::fn_two();
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
}
