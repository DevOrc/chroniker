
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


#[derive(Debug, Copy, Clone, PartialEq)]
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

///Converts two different time units. See the example called "units"
///# Examples
///
///To convert one hour to nanoseconds:
///
///```
///use chroniker::units;
///use chroniker::units::TimeUnit;
///
///let millis_in_hour = units::convert(TimeUnit::Minute, TimeUnit::Nanosecond, 60);
///println!("In one Hour there are {} nanosecond(s)", millis_in_hour);
///```
pub fn convert(from: TimeUnit, to: TimeUnit, value: u64) -> u64{
    let to_nano = to_nano(from, value, to);

    if to_nano.1{
        return to_nano.0;
    }

    from_nano(to, to_nano.0)
}

fn to_nano(from: TimeUnit, value: u64, goal: TimeUnit) -> (u64, bool){
    if from == goal{
        return (value, true)
    }

    match from{
        TimeUnit::Nanosecond => (value, false),
        TimeUnit::Millisecond => to_nano(TimeUnit::Nanosecond, value * 1_000_000, goal),
        TimeUnit::Second => to_nano(TimeUnit::Millisecond, value * 1000, goal),
        TimeUnit::Minute => to_nano(TimeUnit::Second, value * 60, goal),
        TimeUnit::Hour => to_nano(TimeUnit::Minute, value * 60, goal),
        TimeUnit::Day => to_nano(TimeUnit::Hour, value * 24, goal),
        TimeUnit::Week => to_nano(TimeUnit::Day, value * 7, goal),
        TimeUnit::Year => to_nano(TimeUnit::Week, value * 52, goal)
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
    assert!(convert(TimeUnit::Day, TimeUnit::Minute, 5) == 7200);
    assert!(convert(TimeUnit::Day, TimeUnit::Nanosecond, 5) == (4.32e+14) as u64);
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
