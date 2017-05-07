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

    // Do something long and time it.
    do_something_long();
    println!("Duration: {duration}", duration = stopwatch);
    stopwatch.stop();
}

/// This would be the function you actually want to time.
fn do_something_long() {
    use std::io::Write;

    for _ in 0..6 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        print!(".");
        std::io::stdout().flush().unwrap();
    }
    println!();
}
