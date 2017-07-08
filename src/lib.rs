use std::time::Instant;
use std::fmt;
use std::thread;
use std::time::Duration;

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
        write!(f, "{}", self.elapsed_millis())
    }
}

pub fn sleep(time: u64){
    thread::sleep(Duration::from_millis(time));
}
