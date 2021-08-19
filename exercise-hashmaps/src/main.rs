use std::collections::HashMap;

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (String, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the age ("New" or "Used") and miles
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (String, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let mut quality: (String, u32) = ("New".to_string(), 0);

    // Use a conditional expression to check the miles
    // If the car has accumulated miles, then the car is used
    if miles > 0 {
        quality = ("Used".to_string(), miles);
    }

    return quality;
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has closed roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    let car = Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    };

    // Return new instance of "Car" struct
    return car
}

fn main() {
    // Initialize a hash map for car orders
    // - Keys: New or Used, Values: integer
    // - Keys: Manual or Automatic, Values: integer
    let mut orders: HashMap<String, u32> = HashMap::new();
    let (mut new_cars, mut used_cars) = (1, 1);
    let (mut manual, mut auto) = (1, 1);

    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Initialize variables
    let (mut index, mut order) = (1, 1);

    // Declare the car type and initial values
    let mut car: Car;
    let mut miles = 1000; // Start used cars with 1,000 miles
    let mut roof = true;  // convertible = false | hard top = true
    let mut engine: Transmission;

    while order <= 11 {
        // Set car transmission type
        // Set car transmission type, make some roofs convertible
        // When order % 3, swap roof type for fun!
        if order % 3 == 0 {
            engine = Transmission::Automatic;
            roof = !roof;

            orders.insert("Automatic".to_string(), auto);
            auto += 1;
        } else if order % 2 == 0 {
            engine = Transmission::SemiAuto;
        } else {
            engine = Transmission::Manual;

            orders.insert("Manual".to_string(), manual);
            manual += 1;
        }

        // Order the cars, New are even numbers, Used are odd numbers
        if index % 2 != 0 {
            car = car_factory(colors[index - 1].to_string(), engine, roof, miles);

            orders.insert("Used".to_string(), used_cars);
            used_cars += 1;
        } else { 
            car = car_factory(colors[index - 1].to_string(), engine, roof, 0);

            orders.insert("New".to_string(), new_cars);
            new_cars += 1;
        }

        // Display car order details
        // Display car order details by roof type and age of car
        if car.age.0 == "Used" && roof == true {
            println!("{}: {}, {:?}, Closed roof, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1); 
        } else if car.age.0 == "New" && roof == true {
            println!("{}: {}, {:?}, Closed roof, {}", order, car.age.0, car.motor, car.color); 
        } else if car.age.0 == "Used" && roof == false {
            println!("{}: {}, {:?}, Convertible, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1); 
        }else if car.age.0 == "New" && roof == false {
            println!("{}: {}, {:?}, Convertible, {}", order, car.age.0, car.motor, car.color); 
        }

        // Change values for next loop
        order += 1;
        miles += 1000;

        // Adjust the index for the car details
        // Order 11 cars, use index range of 0 -- 4, then repeat from 0
        if index < 4 {
            index = index + 1;
        } else {
            index = 1;
        }
    }

    // Display the hash map of car orders, show <K, V> pairs
    println!("\nCar orders: {:?}", orders);
}