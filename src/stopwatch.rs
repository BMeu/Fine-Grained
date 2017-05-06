// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The actual stopwatch implementation.

use std::fmt;

use time;

/// A stopwatch with lap functionality and nanosecond resolution.
#[derive(Clone, Debug, Default)]
pub struct Stopwatch {
    /// A list of all lap measurements.
    laps: Vec<u64>,

    /// The start time of the currently running lap, or `None` if the stopwatch is not running.
    start_time: Option<u64>,

    /// The sum of all finished laps.
    total_time: u64
}

impl Stopwatch {
    /// Initialize a new stopwatch without starting it.
    pub fn new() -> Stopwatch {
        Stopwatch { laps: vec![], start_time: None, total_time: 0 }
    }

    /// Initialize a new stopwatch and start it.
    pub fn start_new() -> Stopwatch {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        stopwatch
    }

    /// Start the stopwatch.
    pub fn start(&mut self) {
        self.start_time = Some(time::precise_time_ns());
    }

    /// Start a new lap. Save the last lap's time and return it.
    ///
    /// If the stopwatch has not been started, the lap will not be saved and `0` will be returned.
    pub fn lap(&mut self) -> u64 {
        // Determine this lap's duration. If the stopwatch has not been started, the lap is meaningless.
        let current_time: u64 = time::precise_time_ns();
        let lap: u64 = match self.start_time {
            Some(t) => current_time - t,
            None => return 0
        };

        // Add this lap's duration to the total time, add this lap to the list of laps,
        // and reset the starting time for the new lap.
        self.total_time += lap;
        self.laps.push(lap);
        self.start_time = Some(current_time);

        lap
    }

    /// Stop the stopwatch, without updating the total time.
    ///
    /// This will not reset the stopwatch, i.e. the total time and the laps will be preserved.
    pub fn stop(&mut self) {
        self.start_time = None;
    }

    /// Re-initialize the stopwatch without restarting it.
    pub fn reset(&mut self) {
        self.laps = vec![];
        self.start_time = None;
        self.total_time = 0;
    }

    /// Re-initialize the stopwatch and start it.
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    /// Get the total time the stopwatch has been running.
    ///
    /// If the stopwatch is still running, the total time is the time from starting the
    /// stopwatch until now. Otherwise, it is the sum of all laps.
    pub fn total_time(&self) -> u64 {
        match self.start_time {
            Some(current_lap_start_time) => {
                /// If the stopwatch is currently running, the total time is the saved total time
                /// plus the current lap's duration up to this point.
                let current_time: u64 = time::precise_time_ns();
                let lap: u64 = current_time - current_lap_start_time;
                self.total_time + lap
            },
            None => self.total_time
        }
    }

    /// Get the list of all measured lap times in the order the laps were timed.
    pub fn laps(&self) -> &Vec<u64> {
        &self.laps
    }

    /// Get the number of measured laps.
    pub fn number_of_laps(&self) -> usize {
        self.laps.len()
    }

    /// Determine if the stopwatch is currently running.
    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }
}

impl fmt::Display for Stopwatch {
    /// Formats the total time using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{total_time}ns", total_time = self.total_time())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_results)]

    use super::Stopwatch;

    #[test]
    fn new() {
        let stopwatch = Stopwatch::new();
        assert_eq!(stopwatch.laps, vec![]);
        assert_eq!(stopwatch.start_time, None);
        assert_eq!(stopwatch.total_time, 0);
    }

    #[test]
    fn start_new() {
        let stopwatch = Stopwatch::start_new();
        assert_eq!(stopwatch.laps, vec![]);
        assert!(stopwatch.start_time.is_some());
        assert_eq!(stopwatch.total_time, 0);
    }

    #[test]
    fn start() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        assert!(stopwatch.start_time.is_some());
    }

    #[test]
    fn lap() {
        let mut stopwatch = Stopwatch::new();
        let lap_0: u64 = stopwatch.lap();
        assert_eq!(lap_0, 0);
        assert_eq!(stopwatch.laps.len(), 0);
        assert_eq!(stopwatch.total_time, lap_0);

        stopwatch.start();
        let lap_1: u64 = stopwatch.lap();
        assert!(lap_1 > 0);
        assert_eq!(stopwatch.laps.len(), 1);
        assert_eq!(stopwatch.laps[0], lap_1);
        assert_eq!(stopwatch.total_time, lap_1);

        let lap_2: u64 = stopwatch.lap();
        assert!(lap_2 > 0);
        assert_eq!(stopwatch.laps.len(), 2);
        assert_eq!(stopwatch.laps[0], lap_1);
        assert_eq!(stopwatch.laps[1], lap_2);
        assert_eq!(stopwatch.total_time, lap_1 + lap_2);
    }

    #[test]
    fn stop() {
        let mut stopwatch = Stopwatch::start_new();
        let lap: u64 = stopwatch.lap();
        stopwatch.stop();
        assert!(stopwatch.start_time.is_none());
        assert_eq!(stopwatch.laps.len(), 1);
        assert_eq!(stopwatch.total_time, lap);
    }

    #[test]
    fn reset() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        stopwatch.reset();
        assert_eq!(stopwatch.laps, vec![]);
        assert_eq!(stopwatch.start_time, None);
        assert_eq!(stopwatch.total_time, 0);
    }

    #[test]
    fn restart() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        stopwatch.restart();
        assert_eq!(stopwatch.laps, vec![]);
        assert!(stopwatch.start_time.is_some());
        assert_eq!(stopwatch.total_time, 0);
    }

    #[test]
    fn total_time() {
        let mut stopwatch = Stopwatch::start_new();
        let start_time: u64 = stopwatch.start_time.unwrap();
        let mut total_time: u64 = stopwatch.total_time();
        assert!(total_time > 0);
        assert_eq!(stopwatch.total_time, 0);
        assert_eq!(stopwatch.laps, vec![]);
        assert_eq!(stopwatch.start_time.unwrap(), start_time);

        stopwatch.lap();
        stopwatch.stop();
        total_time = stopwatch.total_time();
        assert_eq!(total_time, stopwatch.total_time);
    }

    #[test]
    fn laps() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        stopwatch.lap();
        stopwatch.lap();

        assert_eq!(stopwatch.laps(), &stopwatch.laps);
    }

    #[test]
    fn number_of_laps() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        stopwatch.lap();
        stopwatch.lap();

        assert_eq!(stopwatch.number_of_laps(), 3);
    }

    #[test]
    fn is_running() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());

        stopwatch.start();
        assert!(stopwatch.is_running());
    }

    #[test]
    fn fmt_display() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.total_time = 42;
        assert_eq!(format!("{stopwatch}", stopwatch = stopwatch), "42ns");
    }
}
