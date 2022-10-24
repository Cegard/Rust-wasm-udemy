
fn main() {
    let float_num: f32 = 3.1459; // By default floating numbers are of 64 bits
    let float_num_2 = 64.0;
    println!("{float_num} is a f32 number");
    println!("{float_num_2} is a f64 number");

    // tuples
    let the_tuple: (i8, &str, f32) = (64, "real", 3.1459);
    println!("positions in tuples are accessed by dot .");
    println!("{} {} {}", the_tuple.0, the_tuple.1, the_tuple.2);

    let (zero, one, two) = the_tuple; // unfold
    println!("{zero} {one} {two}");

    let x = [1, 4, 35, 67, 8];
    println!("arrays are accessed by [] {} (just to test {{}})", x[4]);

    let y = [0; 5]; // another way to declare arrays
    println!("arrays are accessed by [] y[4] => {} ", y[4]);
}
