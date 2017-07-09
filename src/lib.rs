use std::fmt;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};

pub mod units;

/// A thin wrapper behind the Instant api from [std::time](https://doc.rust-lang.org/nightly/std/time/).
///The timer is for timing different parts of a program
pub struct Timer{
    /// The actual timer See [std::time::Instant](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html)
    pub time: Instant
}

impl Timer{

    ///Creates and starts the timer.
    ///
    ///# Examples
    ///
    ///```
    ///use chroniker::Timer;
    ///
    ///let timer = Timer::new();
    ///```
    pub fn new() -> Timer{
        Timer{time: Instant::now()}
    }

    ///Similar to elapsed_millis. Gets the elapsed time in the unit passed.
    ///# Examples
    ///
    ///```
    ///use chroniker::Timer;
    ///use chroniker::units::TimeUnit;
    ///
    ///let timer = Timer::new();
    ///
    ///chroniker::sleep(1000);
    ///
    ///let elapsed_sec = timer.get_elapsed(TimeUnit::Second);
    ///println!("Timer: {}", elapsed_sec); //Prints "1"
    ///```
    pub fn get_elapsed(&self, unit: units::TimeUnit) -> u64{
        units::convert(units::TimeUnit::Millisecond, unit, self.elapsed_millis())
    }

    ///Returns the amount of milliseconds since the
    ///timer was created or reset.
    ///# Examples
    ///
    ///```
    ///use chroniker::Timer;
    ///
    /// //Create the timer
    ///let timer = Timer::new();
    ///
    /// //Do some "operations"
    ///chroniker::sleep(1000);
    ///
    ///let elapsed_millis = timer.elapsed_millis();
    ///println!("Timer: {}", elapsed_millis);//Prints 1000
    ///```
    pub fn elapsed_millis(&self) -> u64{
        (self.time.elapsed().as_secs() * 1000) + ((self.time.elapsed().subsec_nanos() / 1_000_000) as u64)
    }

    ///Resets thes timer to 0 and starts
    ///counting up again.
    ///# Examples
    ///
    ///```
    ///use chroniker::Timer;
    ///
    ///let mut timer = Timer::new();
    ///chroniker::sleep(1000);
    ///
    ///let elapsed_millis = timer.elapsed_millis();
    ///println!("Timer: {}", elapsed_millis);//Prints 1000
    ///
    /// //Reset the timer to 0
    ///timer.reset();
    ///println!("Timer: {:?}", timer); //Prints 0
    ///```
    pub fn reset(&mut self){
        self.time = Instant::now();
    }

}

impl fmt::Debug for Timer {

    ///Prints out the time.
    ///# Format
    ///
    ///If the time is 1500 milliseconds (1.5 seconds) it will print out
    ///
    ///```command_line
    ///$ 1.500
    ///```
    ///
    ///If the time is 75699 milliseconds (1 minute, 15 seconds, and 699 milliseconds) it will print out
    ///
    ///```command_line
    ///$ 1:15.699
    ///```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let millis: f64 = (self.elapsed_millis() % 1000) as f64 / 1000.0;
        let mut seconds = self.elapsed_millis() / 1000;
        let mut minutes = seconds / 60;

        while seconds >= 60{
            seconds -= 60;
            minutes += 1;
        }

        //Convert the number into a string and skip then "0." part of "0.99999"
        let milli_vec: Vec<char> = millis.to_string().chars().skip(2).collect();
        let milli_str: String = milli_vec.iter().cloned().collect();//Turn the vector into a string

        if minutes != 0{
            return write!(f, "{}:{}.{}", minutes, seconds, milli_str);
        }

        write!(f, "{}.{}", seconds, milli_str)
    }
}

///Sleeps the thread for the supplied amount of time in
/// milliseconds.
///# Examples
///To sleep for one second:
///
///```
///chroniker::sleep(1000);
///```
///
///However to sleep for 5 seconds you can use
///the convert function:
///
///```
///use chroniker::units;
///use chroniker::units::TimeUnit;
///
///chroniker::sleep(units::convert(TimeUnit::Second, TimeUnit::Millisecond, 5));
///```
pub fn sleep(time: u64){
    thread::sleep(Duration::from_millis(time));
}

///Returns the current time in milliseconds.
///# Examples
///
///```
///
///let unix_time = chroniker::current_time_millis();
///```
pub fn current_time_millis() -> u64{
    let start = SystemTime::now();

    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
