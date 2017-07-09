extern crate chroniker;

use chroniker::units;
use chroniker::units::TimeUnit;

//To run this example: cargo run --example units

fn main(){
    //Lets see how many Nanoseconds are in each unit
    let units = [TimeUnit::Nanosecond, TimeUnit::Millisecond, TimeUnit::Second, TimeUnit::Minute,
        TimeUnit::Hour, TimeUnit::Day, TimeUnit::Week, TimeUnit::Year];

    for unit in 0..units.len() {
        let nano_per_unit = units::convert(units[unit], TimeUnit::Nanosecond, 1);
        println!("In one {:?} there are {} nanosecond(s)", units[unit], nano_per_unit);
    }
}
