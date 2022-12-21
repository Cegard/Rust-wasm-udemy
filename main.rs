
fn main() {
    let mut message = String::from("Hello");
    let message2: &mut String = &mut message;

    message2.push_str(" World!");

    //println!("{}", message); not allowed
    println!("{}", message2);
    // println!("{}", message); //allowed after using message2
}
 