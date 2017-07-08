extern crate chroniker;

use chroniker::Timer;

fn main() {
    let timer = Timer::new();

    chroniker::sleep(500);

    println!("Time: {:?}", timer);

    chroniker::sleep(250);

    println!("Time: {:?}", timer);

    chroniker::sleep(250);

    println!("Time: {:?}", timer);

}
