
fn main() {
    let greeting;
    greeting = "Hello World";
    println!("{}", greeting);

    let ten = 6;
    println!("{} is not ten", ten);
    let ten = 10;
    println!("Now {ten} is ten after shadowing.");

    let mut bye = "";
    println!("{bye}");
    bye = "Bye!";
    println!("{bye}");
}