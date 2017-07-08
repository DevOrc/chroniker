extern crate chroniker;

use chroniker::Timer;

//To run this example: cargo run --example getting_started

fn main() {
    //Creates and starts the timer
    let timer = Timer::new();

    //Do some stuff
    chroniker::sleep(1100);

    //Print the time it took to run
    println!("Time: {:?}", timer);

    //Do some more operations
    chroniker::sleep(250);

    //Print the results again
    println!("Time: {:?}", timer);

    //Now lets try to reset the results.
    //You can do this by recreating the struct
    //or calling the reset method.
    let mut timer = Timer::new();

    //Do some operations
    chroniker::sleep(250);

    println!("Time: {:?}", timer);

    //Lets reset again
    //Note: Unlike all other functions
    // this requires a mutable timer
    timer.reset();

    //Do some operations
    chroniker::sleep(1750);

    println!("Time: {:?}", timer);
}
