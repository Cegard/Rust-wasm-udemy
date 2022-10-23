
fn main() {
    let is_true: bool = true;
    println!("{is_true} comes from a bool var.");

    let max_i8_val: i8 = 127;
    let min_i8_val: i8 = -128;
    println!("{min_i8_val} and {max_i8_val} come from 'i8' number type variables.");

    let bigger: u128 = 100000000000000000000000000000000;
    println!("{bigger} fits in a u128 number type.");

    let arch_size: usize = 18446744073709551615;
    let explicit_sixtyfour: u64 = 18446744073709551615;
    println!(
        "{} is a 'usize' number type and is the same as an 'u64' like {}",
        arch_size,
        explicit_sixtyfour
    );
    println!("and that's because of the most CPU's architecture nowadays");
}
