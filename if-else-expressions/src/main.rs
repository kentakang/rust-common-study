fn main() {
    if 1 == 2 {
        println!("True, the numbers are equal.");
    } else {
        println!("False, the numbers are not equal.");
    }

    let formal = true;
    // maybe instead of ternary operator? i like this
    let greeting = if formal {
        "Good day to you."
    } else {
        "Hey!"
    };

    let num = 500;
    let out_of_range: bool;

    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
}