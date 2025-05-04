/// Step-by-step breakdown of how `f32` is stored in memory and how to extract its components

/// Helper function to print 32 bits in a readable format: 8-bit chunks
fn print_bits_formatted(bits: u32) {
    let bin_str = format!("{:032b}", bits);
    for (i, c) in bin_str
        .chars()
        .enumerate()
    {
        print!("{}", c);
        if i % 8 == 7 && i != 31 {
            print!(" ");
        }
    }
    println!();
}

fn main() {
    // STEP 1: Start with a floating-point number
    let n: f32 = 42.42;
    println!("Original float: {n}");

    // STEP 2: Convert the float to raw u32 bits
    let n_bits: u32 = n.to_bits();
    println!("Bits (raw u32): {n_bits}");
    print!("Bits (binary):   ");
    print_bits_formatted(n_bits);

    // STEP 3: Extract the sign bit (bit 31)
    let sign_bit: u32 = n_bits >> 31;
    println!("Sign bit: {}", sign_bit); // 0 = positive, 1 = negative

    // STEP 4: Extract the exponent bits (bits 23..=30)
    let exponent_bits: u32 = (n_bits >> 23) & 0xff;
    let exponent: i32 = (exponent_bits as i32) - 127; // IEEE 754 bias = 127
    println!("Stored exponent (biased): {}", exponent_bits);
    println!("Actual exponent: {}", exponent);

    // STEP 5: Extract the mantissa (bits 0..=22)
    let mantissa_bits: u32 = n_bits & 0x7FFFFF; // 23 bits
    println!("Mantissa bits: {:023b}", mantissa_bits);
}
