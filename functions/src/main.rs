fn main() {
    println!("Hello, world!");
    is_divisible_by(12, 4);
    is_divisible_by(13, 5);
    is_divisible_by(14, 0);

    if is_input_zero(0) {
        println!("input is zero");
    }

    goodbye();
}

// Basic Function
fn goodbye() {
    println!("Goodbye!");
}

// Function with input arguments
fn is_divisible_by(dividend: u32, divisor: u32) {
    if divisor == 0 {
        println!("Error! Division by zero is not allowed.");
    } else if dividend % divisor > 0 {
        println!("{} % {} has a remainder of {}.", dividend, divisor, (dividend % divisor));
    } else {
        println!("{} % {} has no remainder.", dividend, divisor);
    }
}

// Function with return values
fn is_input_zero(input: u8) -> bool {
    if input == 0 {
        return true;
    }

    false
}