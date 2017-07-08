use std::time::Instant;
use std::fmt;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub mod units;

pub struct Timer{
    time: Instant
}

impl Timer{
    pub fn new() -> Timer{
        Timer{time: Instant::now()}
    }

    pub fn elapsed_millis(&self) -> u64{
        (self.time.elapsed().as_secs() * 1000) + ((self.time.elapsed().subsec_nanos() / 1_000_000) as u64)
    }

    pub fn reset(&mut self){
        self.time = Instant::now();
    }

}

impl fmt::Debug for Timer {
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

pub fn sleep(time: u64){
    thread::sleep(Duration::from_millis(time));
}

pub fn current_time_millis() -> u64{
    let start = SystemTime::now();

    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
