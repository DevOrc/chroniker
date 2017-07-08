extern crate chroniker;

//To run this example: cargo run --example millis_time

fn main() {
    //Get the start time
    let time1: u64 = chroniker::current_time_millis();
    println!("T1: {}", time1);

    //Do stuff
    chroniker::sleep(1000);

    //Get the end time
    let time2: u64 = chroniker::current_time_millis();
    println!("T2: {}", time2);

    //Calculate the difference
    println!("Diff: {}-{} = {}", time1, time2, time2 - time1);
}
