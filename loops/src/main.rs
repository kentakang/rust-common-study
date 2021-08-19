fn main() {
    // Infinite looooooooooop
    /* loop {
        println!("looooooooop fooooooorever!");
        // Break a loop
        break;
    } */

    // break with value
    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;

        if counter > 100 {
            break counter;
        }
    };

    println!("Break the loop at counter = {}.", stop_loop);

    /* while counter < 5 {
        println!("We loop a while...");
        counter += 1;
    } */

    let big_birds = ["ostrich", "peacock", "stork"];

    for bird in big_birds {
        println!("{}", bird);
    }

    for number in 0..5 {
        println!("{}", number * 2);
    }
}
