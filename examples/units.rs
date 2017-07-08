extern crate chroniker;

use chroniker::units;
use chroniker::units::TimeUnit;

//To run this example: cargo run --example units

fn main(){
    //Lets see how many Nanoseconds are in each unit
    let units = [TimeUnit::Nanosecond, TimeUnit::Millisecond, TimeUnit::Second, TimeUnit::Minute];

    for unit in 0..4 {
        let nano_per_unit = units::convert(units[unit], TimeUnit::Nanosecond, 1);
        println!("In one {:?} there are {} nanosecond(s)", units[unit], nano_per_unit);
    }

    //Now Lets see how many millseconds in an hour
    let millis_in_hour = units::convert(TimeUnit::Minute, TimeUnit::Nanosecond, 60);

    println!("In one Hour there are {} nanosecond(s)", millis_in_hour);


}
