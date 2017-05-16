// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

extern crate fine_grained;

use std::thread;
use std::time::Duration;

use fine_grained::Stopwatch;

#[test]
fn repetitive_measurements() {
    let sleep_in_ms: u64 = 50;
    let number_of_rounds: u64 = 10;
    let mut stopwatch = Stopwatch::start_new();

    for _ in 0..number_of_rounds {
        thread::sleep(Duration::from_millis(sleep_in_ms));
        stopwatch.lap();
    }
    let stopwatch = stopwatch.stop();

    let mut measured_total: u64 = 0;
    for lap in stopwatch.laps() {
        assert!(lap / 1_000_000 >= sleep_in_ms);
        measured_total += *lap;
    }

    assert_eq!(stopwatch.number_of_laps() as u64, number_of_rounds);
    assert_eq!(stopwatch.total_time(), measured_total);
    assert!(stopwatch.total_time() / 1_000_000 >= sleep_in_ms * number_of_rounds);
}
