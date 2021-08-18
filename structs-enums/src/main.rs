// Clasic struct
struct Student {
    name: String,
    level: u8,
    pass: bool
}

// Tuple struct
struct Grades(char, char, char, char, f32);

// Enum
enum WebEvent {
    WELoad,
    WEKeys(String, char),
    WEClick { x: i64, y: i64 }
}

// or with struct
struct KeyPress(String, char);
struct MouseClick { x: i64, y: i64 }
enum WebEventWithoutStruct {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress)
}

fn main() {
    // Instantiate classic struct, specify fields in random order, or in specified order
    let student_1 = Student { name: String::from("Chan Kang"), level: 10, pass: true };
    let student_2 = Student { name: String::from("John Doe"), level: 5, pass: false };
    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'A', 'A', 4.5);
    let mark_2 = Grades('F', 'F', 'A', 'F', 1.2);

    println!("{}, Level: {}, pass: {}\nGrades: {}, {}, {}, {}. Average: {}", student_1.name, student_1.level, student_1.pass, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, Level: {}, pass: {}\nGrades: {}, {}, {}, {}. Average: {}", student_2.name, student_2.level, student_2.pass, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}