
fn main() {
    let message = String::from("Hello");
    let message_two = extend_message(message);

    println!("{}", message_two);
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World!");
    return a;
}
 