# Fine-Grained

[![Build Status](https://travis-ci.org/BMeu/Fine-Grained.svg?branch=master)](https://travis-ci.org/BMeu/Fine-Grained)
[![crates.io](https://img.shields.io/crates/v/fine_grained.svg)](https://crates.io/crates/fine_grained)
[![Documentation](https://docs.rs/fine_grained/badge.svg)](https://docs.rs/fine_grained/)

A Rust stopwatch with lap functionality and nanosecond resolution.

## Examples

Get a single measurement:

```rust
extern crate fine_grained;

use fine_grained::Stopwatch;

fn main() {
    // Get a new stopwatch and start it.
    let mut stopwatch = Stopwatch::start_new();

    // Do something long and time it.
    // do_something_long();
    println!("Duration: {duration}ns", duration = stopwatch);
    stopwatch.stop();
}
```

Get measurements for repetitive tasks and a total time:

```rust
extern crate fine_grained;

use fine_grained::Stopwatch;

fn main() {
    // Get a new stopwatch and start it.
    let mut stopwatch = Stopwatch::start_new();

    // Do something repetitive you want to time.
    for _ in 0..10 {
        // do_something_repetitive();
        stopwatch.lap();
    }
    stopwatch.stop();

    // Print the timing results.
    for (i, &lap) in stopwatch.laps().into_iter().enumerate() {
        println!("Round {i}: {duration}ns", i = i, duration = lap);
    }
    println!("Total time: {duration}ns", duration = stopwatch);
}
```

Get measurements for multiple indepedent tasks and a total time:

```rust
extern crate fine_grained;

use fine_grained::Stopwatch;

fn main() {
    // Get a new stopwatch and start it.
    let mut stopwatch = Stopwatch::start_new();

    // Do foo.
    // do_foo();
    let time_to_do_foo: u64 = stopwatch.lap();

    // Do bar.
    // do_bar();
    let time_to_do_bar: u64 = stopwatch.lap();

    // Do foobar.
    // do_foobar();
    let time_to_do_foobar: u64 = stopwatch.lap();

    stopwatch.stop();
    println!("Time to do foo: {duration}ns", duration = time_to_do_foo);
    println!("Time to do bar: {duration}ns", duration = time_to_do_bar);
    println!("Time to do foobar: {duration}ns", duration = time_to_do_foobar);
    println!("Total time: {duration}ns", duration = stopwatch);
}
```

## Inspiration

Inspired by Chucky Ellison's stopwatch (https://github.com/ellisonch/rust-stopwatch).
