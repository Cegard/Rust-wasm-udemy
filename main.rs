
fn main() {
    let greeting = "Hello World";
    println!("{}", greet(greeting));
}

fn greet(message: &str) -> &str {
    println!("{message}");
    
    return "Hi there!";
}