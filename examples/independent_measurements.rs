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

    // Do foo.
    do_foo();
    let time_to_do_foo: u64 = stopwatch.lap();

    // Do bar.
    do_bar();
    let time_to_do_bar: u64 = stopwatch.lap();

    // Do foobar.
    do_foobar();
    let time_to_do_foobar: u64 = stopwatch.lap();

    stopwatch.stop();
    println!("   Time to do foo: {duration}ns", duration = time_to_do_foo);
    println!("   Time to do bar:  {duration}ns", duration = time_to_do_bar);
    println!("Time to do foobar: {duration}ns", duration = time_to_do_foobar);
    println!("       Total time: {duration}", duration = stopwatch);
}

/// This would be the first function you actually want to time.
fn do_foo() {
    use std::io::Write;

    print!("   Foo: ");
    std::io::stdout().flush().unwrap();
    for _ in 0..6 {
        std::thread::sleep(std::time::Duration::from_millis(450));
        print!(".");
        std::io::stdout().flush().unwrap();
    }
    println!();
}

/// This would be the second function you actually want to time.
fn do_bar() {
    use std::io::Write;

    print!("   Bar: ");
    std::io::stdout().flush().unwrap();
    for _ in 0..30 {
        std::thread::sleep(std::time::Duration::from_millis(30));
        print!(".");
        std::io::stdout().flush().unwrap();
    }
    println!();
}

/// This would be the third function you actually want to time.
fn do_foobar() {
    use std::io::Write;

    print!("Foobar: ");
    std::io::stdout().flush().unwrap();
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(300));
        print!(".");
        std::io::stdout().flush().unwrap();
    }
    println!();
}
