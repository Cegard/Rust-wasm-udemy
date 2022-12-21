
fn main() {
    let message = String::from("Hello");
    let message2: &String = &message;

    println!("{}", message);
    println!("{}", message2);
}
 