
#[derive(Debug, Copy, Clone)]
pub enum TimeUnit{
    Nanosecond,
    Millisecond,
    Second,
    Minute
}

///Converts two different time units. See the example called "units"
///# Examples
///
///To convert one hour to nanoseconds:
///
///```
///let millis_in_hour = units::convert(TimeUnit::Minute, TimeUnit::Nanosecond, 60);
///println!("In one Hour there are {} nanosecond(s)", millis_in_hour);
///```
pub fn convert(from: TimeUnit, to: TimeUnit, value: u64) -> u64{
    from_nano(to, to_nano(from, value))
}

fn to_nano(from: TimeUnit, value: u64) -> u64{
    match from{
        TimeUnit::Nanosecond => value,
        TimeUnit::Millisecond => value * 1_000_000,
        TimeUnit::Second => to_nano(TimeUnit::Millisecond, value * 1000),
        TimeUnit::Minute => to_nano(TimeUnit::Second, value * 60)
    }
}

fn from_nano(to: TimeUnit, value: u64) -> u64{
    match to{
        TimeUnit::Nanosecond => value,
        TimeUnit::Millisecond => value / 1_000_000,
        TimeUnit::Second => from_nano(TimeUnit::Millisecond, value / 1000),
        TimeUnit::Minute => from_nano(TimeUnit::Second, value / 60)
    }
}

#[test]
fn test_convert(){
    assert!(convert(TimeUnit::Millisecond, TimeUnit::Second, 3000) == 3);
    assert!(convert(TimeUnit::Millisecond, TimeUnit::Nanosecond, 1) == 1_000_000);
    assert!(convert(TimeUnit::Minute, TimeUnit::Nanosecond, 2) == 120_000_000_000);
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
