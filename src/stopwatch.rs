// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The actual stopwatch implementation.
//!
//! See the [crate documentation](../index.html) for examples.

use std::fmt;
use std::marker::PhantomData;

use time;

/// A unit-like struct for marking a stopwatch as initialized.
#[derive(Clone, Copy, Debug)]
struct Initialized;

/// A unit-like struct for marking a stopwatch as running.
#[derive(Clone, Copy, Debug)]
struct Running;

/// A unit-like struct for marking a stopwatch as paused.
#[derive(Clone, Copy, Debug)]
struct Paused;

/// A unit-like struct for marking a stopwatch as stopped.
#[derive(Clone, Copy, Debug)]
struct Stopped;

/// A stopwatch with lap functionality and nanosecond resolution.
///
/// See the [crate documentation](index.html) for examples.
#[derive(Clone, Debug, Default)]
pub struct Stopwatch<State> {
    /// A list of all lap measurements.
    laps: Vec<u64>,

    /// The start time of the currently running lap, or `None` if the stopwatch is not running.
    start_time: Option<u64>,

    /// The sum of all finished laps.
    total_time: u64,

    /// The state of the stopwatch.
    state: PhantomData<State>,
}

impl<State> Stopwatch<State> {
    /// Determine if the stopwatch is currently running.
    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }

    // TODO: Implement `IntoIterator` instead for iterating over the laps.
    /// Get the list of all measured lap times in the order the laps were timed.
    pub fn laps(&self) -> &Vec<u64> {
        &self.laps
    }

    /// Get the number of measured laps.
    pub fn number_of_laps(&self) -> usize {
        self.laps.len()
    }

    /// Get the total time the stopwatch has been running.
    ///
    /// If the stopwatch is still running, the total time is the time from starting the
    /// stopwatch until now. Otherwise, it is the sum of all laps.
    pub fn total_time(&self) -> u64 {
        match self.start_time {
            Some(current_lap_start_time) => {
                // If the stopwatch is currently running, the total time is the saved total time plus the current lap's
                // duration up to this point.
                let current_time: u64 = time::precise_time_ns();
                let lap: u64 = current_time - current_lap_start_time;
                self.total_time + lap
            },
            None => self.total_time
        }
    }
}

impl Stopwatch<Initialized> {
    /// Initialize a new stopwatch without starting it.
    pub fn new() -> Stopwatch<Initialized> {
        Stopwatch {
            laps: Vec::new(),
            start_time: None,
            total_time: 0,
            state: PhantomData::<Initialized>,
        }
    }

    /// Start the stopwatch.
    pub fn start(self) -> Stopwatch<Running> {
        Stopwatch {
            laps: self.laps,
            start_time: Some(time::precise_time_ns()),
            total_time: self.total_time,
            state: PhantomData::<Running>,
        }
    }
}

impl Stopwatch<Running> {
    /// Initialize a new stopwatch and start it.
    ///
    /// This an alias for [`Stopwatch::new()`](#method.new)[`.start()`](#method.start)
    pub fn start_new() -> Stopwatch<Running> {
        Stopwatch::new().start()
    }

    /// Start a new lap. Save the last lap's time and return it.
    pub fn lap(&mut self) -> u64 {
        let lap: u64 = self.finish_current_lap();
        self.start_time = Some(time::precise_time_ns());
        lap
    }

    /// Finish the current lap and immediately pause the stopwatch.
    pub fn lap_and_pause(mut self) -> (u64, Stopwatch<Paused>) {
        let lap: u64 = self.finish_current_lap();

        // Insert an empty lap into the list. This will be removed on resume.
        self.laps.push(0);

        let stopwatch = Stopwatch {
            laps: self.laps,
            start_time: None,
            total_time: self.total_time,
            state: PhantomData::<Paused>,
        };

        (lap, stopwatch)
    }

    /// Finish the current lap and immediately stop the stopwatch.
    pub fn lap_and_stop(mut self) -> (u64, Stopwatch<Stopped>) {
        let lap: u64 = self.finish_current_lap();
        (lap, self.stop())
    }

    /// Pause the stopwatch.
    ///
    /// The current lap is inserted into the list of laps with its duration at this time.
    pub fn pause(mut self) -> Stopwatch<Paused> {
        // Store how long the current lap has been running so far.
        let lap: u64 = self.get_current_laps_duration();
        self.laps.push(lap);

        Stopwatch {
            laps: self.laps,
            start_time: None,
            total_time: self.total_time,
            state: PhantomData::<Paused>,
        }
    }

    /// Stop the stopwatch.
    pub fn stop(self) -> Stopwatch<Stopped> {
        Stopwatch {
            laps: self.laps,
            start_time: None,
            total_time: self.total_time,
            state: PhantomData::<Stopped>,
        }
    }

    /// Finish the current lap: get its duration and add it to the list of laps and the total time.
    #[inline(always)]
    fn finish_current_lap(&mut self) -> u64 {
        let lap: u64 = self.get_current_laps_duration();
        self.total_time += lap;
        self.laps.push(lap);
        lap
    }

    /// Get the current lap's duration up to this point..
    #[inline(always)]
    fn get_current_laps_duration(&self) -> u64 {
        // Determine this lap's duration.
        let current_time: u64 = time::precise_time_ns();
        match self.start_time {
            Some(time) => current_time - time,
            None => unreachable!()
        }
    }
}

impl Stopwatch<Paused> {
    /// Resume the stopwatch.
    ///
    /// If a lap has been paused as well (i.e. [`pause()`](#method.pause) has been called), this lap will be resumed.
    pub fn resume(mut self) -> Stopwatch<Running> {
        let paused_lap: u64 = match self.laps.pop() {
            Some(duration) => duration,
            None => unreachable!()
        };
        Stopwatch {
            laps: self.laps,
            // The start time of the paused lap dates back to the current time minus the paused lap's duration.
            start_time: Some(time::precise_time_ns() - paused_lap),
            total_time: self.total_time,
            state: PhantomData::<Running>,
        }
    }

    /// Stop the stopwatch.
    ///
    /// If a lap has been paused as well, (i.e. [`pause()`](#method.pause) has been called), this lap will be stopped.
    pub fn stop(mut self) -> Stopwatch<Stopped> {
        // If the last lap's duration is `0`, there is no paused lap (happens if `lap_and_pause()` has been called).
        let paused_lap: u64 = match self.laps.pop() {
            Some(0) => 0,
            Some(paused_lap) => {
                self.laps.push(paused_lap);
                paused_lap
            },
            None => unreachable!(),
        };
        Stopwatch {
            laps: self.laps,
            start_time: None,
            total_time: self.total_time + paused_lap,
            state: PhantomData::<Stopped>
        }
    }
}

impl Stopwatch<Stopped> {
    /// Re-initialize the stopwatch without restarting it.
    ///
    /// This is an alias for [`Stopwatch::new()`](#method.new).
    pub fn reset(self) -> Stopwatch<Initialized> {
        Stopwatch::new()
    }

    /// Re-initialize the stopwatch and start it.
    ///
    /// This is an alias for [`Stopwatch::start_new()`](#method.start_new).
    pub fn restart(self) -> Stopwatch<Running> {
        Stopwatch::start_new()
    }
}

impl<State> fmt::Display for Stopwatch<State> {
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
        let stopwatch = Stopwatch::new();
        let stopwatch = stopwatch.start();
        assert!(stopwatch.start_time.is_some());
    }

    #[test]
    fn lap() {
        let mut stopwatch = Stopwatch::start_new();
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
        let stopwatch = stopwatch.stop();
        assert!(stopwatch.start_time.is_none());
        assert_eq!(stopwatch.laps.len(), 1);
        assert_eq!(stopwatch.total_time, lap);

        let mut stopwatch = Stopwatch::start_new();
        let lap: u64 = stopwatch.lap();
        let stopwatch = stopwatch.pause();
        let stopwatch = stopwatch.stop();
        assert!(stopwatch.start_time.is_none());
        assert_eq!(stopwatch.laps.len(), 2);
        assert!(stopwatch.total_time > lap);

        let stopwatch = Stopwatch::start_new();
        let (lap, stopwatch) = stopwatch.lap_and_pause();
        let stopwatch = stopwatch.stop();
        assert!(stopwatch.start_time.is_none());
        assert_eq!(stopwatch.laps.len(), 1);
        assert_eq!(stopwatch.total_time, lap);
    }

    #[test]
    fn reset() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        let stopwatch = stopwatch.stop();
        let stopwatch = stopwatch.reset();
        assert_eq!(stopwatch.laps, vec![]);
        assert_eq!(stopwatch.start_time, None);
        assert_eq!(stopwatch.total_time, 0);
    }

    #[test]
    fn restart() {
        let mut stopwatch = Stopwatch::start_new();
        stopwatch.lap();
        let stopwatch = stopwatch.stop();
        let stopwatch = stopwatch.restart();
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
        let stopwatch = stopwatch.stop();
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
        let stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());

        let stopwatch = stopwatch.start();
        assert!(stopwatch.is_running());
    }

    #[test]
    fn fmt_display() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.total_time = 42;
        assert_eq!(format!("{stopwatch}", stopwatch = stopwatch), "42ns");
    }
}
