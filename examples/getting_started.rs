extern crate chroniker;

use chroniker::Timer;

//To run this example: cargo run --example getting_started

fn main() {
    //Creates and starts the timer
    let timer = Timer::new();

    //Do some more operations
    chroniker::sleep(1001);

    //Print the results
    println!("Time: {:?}", timer);

    //Now lets try to reset the results.
    //You can do this by recreating the struct
    //or calling the reset method.
    let mut timer = Timer::new();

    //Do some operations
    chroniker::sleep(250);

    println!("Time: {:?}", timer);

    //Lets reset again
    timer.reset();

    //Do some operations
    chroniker::sleep(1750);

    println!("Time: {} ms", timer.elapsed_millis());
}
