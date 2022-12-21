
fn main() {
    let mut a = 0;
    let d = a; // storing var a on a inmuable variable

    let mut b = &mut a; // first reference
    let c = &mut b; // second reference

    **c = 100; // dereferencing twice to access var a's value
    println!("{}", d == **c);
}
 