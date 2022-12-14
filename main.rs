
fn main() {
    let message = String::from("Hello");
    print_message(message); // message is moved to the function 
    // calling the variable message after this is invalid
    // println!("{}", message);
}

fn print_message(a: String) {
    println!("{}", a);
    let c = a; // moving a into c
}
