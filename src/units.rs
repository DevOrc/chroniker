
//! The units module is the most important module in the chroniker library. Most other functions rely on
//! this module. All it does is convert different TimeUnits.
//!# Examples
//!For instance to covert 15 minutes to milliseconds you would need to run
//!
//!```
//!use chroniker::units;
//!use chroniker::units::TimeUnit;
//!
//!let mins_in_mills = units::convert(TimeUnit::Minute, TimeUnit::Millisecond, 15);
//!```
//!


#[derive(Debug, Copy, Clone)]
///The different Units supported by the chroniker library
pub enum TimeUnit{
    ///One billionth of a second.
    Nanosecond,
    /// One thousandth of a second
    Millisecond,
    ///One full second
    Second,
    ///60 Seconds
    Minute,
    ///60 Minutes
    Hour,
    ///24 Hours
    Day,
    /// 7 Days
    Week,
    /// 52 Weeks
    Year
}

impl TimeUnit{

    ///Converts a unit to another unit. See the example called "units"
    ///# Examples
    ///
    ///To convert one hour to nanoseconds:
    ///
    ///```
    ///use chroniker::units::TimeUnit;
    ///
    ///let nano_in_hour = TimeUnit::Hour.to(TimeUnit::Nanosecond, 1);
    ///println!("In one Hour there are {} nanosecond(s)", nano_in_hour);
    ///```
    pub fn to(&self, to: TimeUnit, amount: u64) -> u64{
        convert(*self, to, amount)
    }

}

///Converts two different time units. See the example called "units"
///# Examples
///
///To convert one hour to nanoseconds:
///
///```
///use chroniker::units;
///use chroniker::units::TimeUnit;
///
///let nano_in_hour = units::convert(TimeUnit::Minute, TimeUnit::Nanosecond, 60);
///println!("In one Hour there are {} nanosecond(s)", nano_in_hour);
///```
pub fn convert(from: TimeUnit, to: TimeUnit, value: u64) -> u64{
    from_nano(to, to_nano(from, value))
}

fn to_nano(from: TimeUnit, value: u64) -> u64{
    match from{
        TimeUnit::Nanosecond => value,
        TimeUnit::Millisecond => value * 1_000_000,
        TimeUnit::Second => to_nano(TimeUnit::Millisecond, value * 1000),
        TimeUnit::Minute => to_nano(TimeUnit::Second, value * 60),
        TimeUnit::Hour => to_nano(TimeUnit::Minute, value * 60),
        TimeUnit::Day => to_nano(TimeUnit::Hour, value * 24),
        TimeUnit::Week => to_nano(TimeUnit::Day, value * 7),
        TimeUnit::Year => to_nano(TimeUnit::Week, value * 52)
    }
}

fn from_nano(to: TimeUnit, value: u64) -> u64{
    match to{
        TimeUnit::Nanosecond => value,
        TimeUnit::Millisecond => value / 1_000_000,
        TimeUnit::Second => from_nano(TimeUnit::Millisecond, value / 1000),
        TimeUnit::Minute => from_nano(TimeUnit::Second, value / 60),
        TimeUnit::Hour => from_nano(TimeUnit::Minute, value / 60),
        TimeUnit::Day => from_nano(TimeUnit::Hour, value / 24),
        TimeUnit::Week => from_nano(TimeUnit::Day, value / 7),
        TimeUnit::Year => from_nano(TimeUnit::Week, value / 52)
    }
}

#[test]
fn test_convert(){
    assert!(convert(TimeUnit::Millisecond, TimeUnit::Second, 3000) == 3);
    assert!(convert(TimeUnit::Millisecond, TimeUnit::Nanosecond, 1) == 1_000_000);
    assert!(convert(TimeUnit::Minute, TimeUnit::Nanosecond, 2) == 120_000_000_000);
    assert!(convert(TimeUnit::Day, TimeUnit::Minute, 5) == 7200);
    assert!(convert(TimeUnit::Day, TimeUnit::Nanosecond, 5) == (4.32e+14) as u64);
}

#[test]
fn test_timeunit_impl(){
    assert!(TimeUnit::Millisecond.to(TimeUnit::Second, 3000) == 3);
    assert!(TimeUnit::Millisecond.to(TimeUnit::Nanosecond, 1) == 1_000_000);
    assert!(TimeUnit::Minute.to(TimeUnit::Nanosecond, 2) == 120_000_000_000);
    assert!(TimeUnit::Day.to(TimeUnit::Minute, 5) == 7200);
    assert!(TimeUnit::Day.to(TimeUnit::Nanosecond, 5) == (4.32e+14) as u64);
}

#[test]
fn test_to_nano(){
    //Convert Units
    assert!(to_nano(TimeUnit::Millisecond, 1) == 1_000_000);
    assert!(to_nano(TimeUnit::Second, 1) == 1_000_000_000);
    assert!(to_nano(TimeUnit::Minute, 1) == 60_000_000_000);

    //Handles a non-one input
    assert!(to_nano(TimeUnit::Millisecond, 5) == 5_000_000);
    assert!(to_nano(TimeUnit::Second, 10) == 10_000_000_000);
    assert!(to_nano(TimeUnit::Minute, 2) == 120_000_000_000);
}

#[test]
fn test_from_nano(){
    //Convert Units
    assert!(from_nano(TimeUnit::Millisecond, 1_000_000) == 1);
    assert!(from_nano(TimeUnit::Second, 1_000_000_000) == 1);
    assert!(from_nano(TimeUnit::Minute, 60_000_000_000) == 1);

    //Handles a non-one input
    assert!(from_nano(TimeUnit::Millisecond, 5_000_000) == 5);
    assert!(from_nano(TimeUnit::Second, 10_000_000_000) == 10);
    assert!(from_nano(TimeUnit::Minute, 120_000_000_000) == 2);
}
