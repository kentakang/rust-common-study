fn main() {
    // Declare a variable and set mutable
    let a_number;
    // Declare a second variable and bind the value
    let a_word = "Fourty Two";

    // Assign a value to a_number
    a_number = 42;

    // variable is immutable by default, can't change value of variable in default.
    // a_number = 50;

    println!("The number is {}", a_number);
    println!("The word is {}.", a_word);

    // mutable
    let mut b_number = 42;

    println!("The answer to life the universe and everything is {}.", b_number);

    // We can change of value in mutable
    b_number = 10;

    println!("The answer to life the universe and everything is not {}", b_number);

    // Declare variable with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable shadow_num
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);
}