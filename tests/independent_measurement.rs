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
fn single_measurement() {
    let foo_sleep_in_ms: u64 = 40;
    let bar_sleep_in_ms: u64 = 70;
    let foobar_sleep_in_ms: u64 = 100;
    let mut stopwatch = Stopwatch::start_new();

    thread::sleep(Duration::from_millis(foo_sleep_in_ms));
    let measurement_foo: u64 = stopwatch.lap();

    thread::sleep(Duration::from_millis(bar_sleep_in_ms));
    let measurement_bar: u64 = stopwatch.lap();

    thread::sleep(Duration::from_millis(foobar_sleep_in_ms));
    let measurement_foobar: u64 = stopwatch.lap();

    stopwatch.stop();

    let measured_total: u64 = measurement_foo + measurement_bar + measurement_foobar;
    assert_eq!(measurement_foo / 1_000_000, foo_sleep_in_ms);
    assert_eq!(measurement_bar / 1_000_000, bar_sleep_in_ms);
    assert_eq!(measurement_foobar / 1_000_000, foobar_sleep_in_ms);
    assert_eq!(stopwatch.total_time(), measured_total);

    // Allow a small delta since there is a little overhead.
    let total_time_in_ms: u64 = stopwatch.total_time() / 1_000_000;
    let expected_total_time: u64 = foo_sleep_in_ms + bar_sleep_in_ms + foobar_sleep_in_ms;
    let delta: u64 = if total_time_in_ms > expected_total_time {
        total_time_in_ms - expected_total_time
    } else {
        expected_total_time - total_time_in_ms
    };
    assert!(delta <= 1);
}
