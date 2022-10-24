
fn main() {
    let custom_num = 95_000_000; // underscores don't affect the number
    let hex_num = 0xfa; // a hexadecimal number
    let bin_num = 0b1001010; // a binary number
    let byte_num = b'R'; // UTF encoding (UTF is mainly writen in hex code)

    println!("let custom_num = 95_000_000; \t => \t {custom_num}");
    println!("let hex_num = 0xfa; \t\t => \t {hex_num}");
    println!("let bin_num = 0b1001010; \t => \t {bin_num}");
    println!("let byte_num = b'R'; \t\t => \t {byte_num}");
}
