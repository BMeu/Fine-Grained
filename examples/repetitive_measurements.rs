// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

extern crate fine_grained;

use fine_grained::Stopwatch;

fn main() {
    // Get a new stopwatch and start it.
    let mut stopwatch = Stopwatch::start_new();

    // Do something repetitive you want to time.
    for _ in 0..10 {
        do_something_repetitive();
        stopwatch.lap();
    }
    let stopwatch = stopwatch.stop();

    // The `println!()` is only needed because our repetitive task does not print newlines.
    println!();

    // Print the timing results.
    for (i, &lap) in stopwatch.laps().into_iter().enumerate() {
        println!("   Round {i}:  {duration}ns", i = i, duration = lap);
    }
    println!("Total time: {duration}", duration = stopwatch);
}

/// This would be the function you actually want to time.
fn do_something_repetitive() {
    use std::io::Write;

    std::thread::sleep(std::time::Duration::from_millis(500));
    print!(".");
    std::io::stdout().flush().unwrap();
}
