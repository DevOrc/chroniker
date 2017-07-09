use std::time::Instant;
use std::fmt;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub mod units;

pub struct Timer{
    time: Instant
}

impl Timer{
    ///Creates and starts the timer.
    ///
    ///# Examples
    ///
    ///```
    ///let timer = Timer::new();
    ///```
    pub fn new() -> Timer{
        Timer{time: Instant::now()}
    }

    ///Returns the amount of milliseconds since the
    ///timer was created or reset.
    ///# Examples
    ///
    ///```
    ///let timer = timer::new();
    ///```
    ///Now lets do some operations
    ///
    ///```
    ///chroniker::sleep(1000);
    ///```
    ///
    ///Now lets check out the total time passed.
    ///This will print 1000 or 1001
    ///
    ///```
    ///let elapsed_millis = timer.elapsed_millis();
    ///println!("Timer: {}", elapsed_millis);
    ///```
    pub fn elapsed_millis(&self) -> u64{
        (self.time.elapsed().as_secs() * 1000) + ((self.time.elapsed().subsec_nanos() / 1_000_000) as u64)
    }

    ///Resets thes timer to 0 and starts
    ///counting up again.
    ///# Examples
    ///
    ///```
    ///let mut timer = timer::new();
    ///chroniker::sleep(1000);
    ///```
    ///
    ///Now lets check out the total time passed.
    ///This will print 1000 or 1001
    ///
    ///```
    ///let elapsed_millis = timer.elapsed_millis();
    ///println!("Timer: {}", elapsed_millis);
    ///```
    ///
    ///However, if you reset it and then print out the results,
    ///the program will output 0.000.
    ///
    ///```
    ///timer.reset();
    ///println!("Timer: {:?}", timer);
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
    ///```
    ///$ 1.500
    ///```
    ///
    ///If the time is 75699 milliseconds (1 minute, 15 seconds, and 699 milliseconds) it will print out
    ///
    ///```
    ///$ 1:15.699
    ///```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seconds = 0;
        let mut millis = self.elapsed_millis();
        let mut minutes = 0;

        while millis >= 1000{
            millis -= 1000;
            seconds += 1;
        }

        while seconds >= 60{
            seconds -= 60;
            minutes += 1;
        }

        if minutes != 0{
            return write!(f, "{}:{}.{}", minutes, seconds, millis);
        }

        write!(f, "{}.{}", seconds, millis)
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
///However to sleep for one hour you can use
///the convert function:
///
///```
///use chroniker::unit;
///chroniker::sleep(unit::covert(TimeUnit::Minutes, TimeUnit::Milliseconds, 60));
///```
pub fn sleep(time: u64){
    thread::sleep(Duration::from_millis(time));
}

///Returns the current time in milliseconds.
///# Examples
///
///```
///let unix_time = chroniker::current_time_millis();
///```
pub fn current_time_millis() -> u64{
    let start = SystemTime::now();

    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
