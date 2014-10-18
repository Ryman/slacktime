extern crate slacktime;
extern crate time;
extern crate test;

use slacktime::SlackTimer;
use std::time::Duration;
use std::io::timer::sleep;
use std::sync::{Arc, Future};

fn t(d: Duration) {
    let foo = SlackTimer::new(d);

    range(0, 10u).fold(foo.get_time(), |old, _| {
        println!("old: {}", old)
        sleep(d * 2); // bit of a buffer
        let new = foo.get_time();
        println!("new: {}", new)
        assert!(new > old, "{} was not greater than {}", new, old);

        new
    });
}

#[test]
fn all_increasing_5() {
    t(Duration::milliseconds(5));
}

#[test]
fn all_increasing_50() {
    t(Duration::milliseconds(50));
}

#[test]
fn all_increasing_500() {
    t(Duration::milliseconds(500));
}

#[test]
fn ensure_sharable() {
    let shared = Arc::new(SlackTimer::new(Duration::milliseconds(5)));

    let futures = Vec::from_fn(100000, |i| {
        let timer = shared.clone();
        Future::spawn(proc() timer.get_time())
    });

    for mut future in futures.into_iter() {
        println!("{}", future.get());
    }
}
