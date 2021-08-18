use std::collections::HashMap;

fn main() {
    // Declare array, initialize all values, compiler infers length = 7
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday"
    ];
    // Declare array, first value = "0", length = 5
    // Array is defined as [T; size]
    let bytes = [0; 5];

    println!("{:?}", &days);
    println!("{:?}", &bytes);

    // Array indexing
    let first = days[0];
    let second = days[1];

    println!("First day of week is {}", first);
    println!("Second day of week is {}", second);

    // Vectors
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    // Declare vector, first value = "0", length = 5
    let zeroes = vec![0; 5];
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruits: Vec<&str> = Vec::new();
    
    println!("Initial vector: {:?}", three_nums);
    println!("Zeroes: {:?}", zeroes);

    fruits.push("apple");
    fruits.push("banana");

    println!("Fruits: {:?}", fruits);
    println!("Pop off: {:?}", fruits.pop());

    // Hash maps <key, value>
    let mut reviews: HashMap<&str, &str> = HashMap::new();

    reviews.insert("Ancient Roman History", "Very accurate.");
    reviews.insert("The Hitchhiker's Guide to the Galaxy", "Awesome!");

    // Look for a specific review
    let book: &str = "The Hitchhiker's Guide to the Galaxy";

    println!("Review for {}: {:?}", book, reviews.get(book));

    // Remove review
    let obsolete: &str = "Ancient Roman History";

    println!("{} removed.", obsolete);
    reviews.remove(obsolete);
}
