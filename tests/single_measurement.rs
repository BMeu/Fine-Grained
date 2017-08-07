// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

extern crate fine_grained;

use std::thread;
use std::time::Duration;

use fine_grained::Stopwatch;
use fine_grained::Running;

#[test]
fn single_measurement() {
    let sleep_in_ms: u64 = 500;
    let mut stopwatch = Stopwatch::start_new();

    thread::sleep(Duration::from_millis(sleep_in_ms));
    let measurement: u64 = stopwatch.lap();

    assert!(measurement / 1_000_000 >= sleep_in_ms);
}
