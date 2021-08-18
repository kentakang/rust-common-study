fn main() {
    // Numbers
    let number_64 = 4.0; // float, default type is f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    // Text: Characters
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃'; // Rust char is 21-bit integer

    // Text: Strings
    let string_1 = "miley "; // default type is &str
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}", smiley_face, uppercase_s, string_1, lowercase_f, string_2);

    // Tuple
    let tuple_e = ('e', 5i32, true); // Declare a tuple of three elements
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2); // Use tuple indexing
}
