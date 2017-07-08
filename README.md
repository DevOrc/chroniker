# Chroniker
A simple library to help with time in rust

## Usage
Chroniker has two main functions. The sleep function and the Timer API. However it also has some other utilities. One of them is the function to get the unix time.

#### The Sleep Function
The first, the sleep function is simply there too make sleeping simple. For instance, the next example sleeps the program for 1 second (or 1000 milliseconds).   
```rust
chroniker::sleep(1000);
```

#### The Timer API
The timer API makes timer programs very simple to time different parts of the program. To start the timer you call the timer new function. For instance:
```rust
let timer = chroniker::Timer::new();
```
After that the timer automatically starts. To get the time you have two options. You can either print it or call the elapsed_millis() method to get the amount of milliseconds passed. Continuing on the previous example we get:
```rust
println!("Timer: {:?}", timer);
let time_passed: i64 = timer.elapsed_millis();
```
If you want to reset the timer you will need to call the reset method like this:
```rust
//Create the timer
let mut timer = chroniker::Timer::new();
//Do first operation
foo();
//Print results and then reset the timer
println!("Timer: {:?}", timer);
timer.reset();
//Do second operation
bar();
//Reprint the results
println!("Timer: {:?}", timer);
```
To see a complete version see the "getting_started" example. Run:

## Examples
Most of the other utilities are covered in examples.
#### Getting Started
Covers the Timer API
```
$ cargo run --example getting_started
```
#### Millis Time
Covers how to get the UNIX time
```
$ cargo run --example millis_time
```

## TODO

1. Add more to string functions to the timer implementation
2. Add functions to help convert different time units (nanoseconds, milliseconds, seconds, minutes, etc)
