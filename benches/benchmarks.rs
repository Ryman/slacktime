extern crate slacktime;
extern crate time;
extern crate test;

use slacktime::SlackTimer;
use std::time::Duration;
use std::sync::{Arc};

#[bench]
fn stdlib(b: &mut test::Bencher) {
    b.iter(|| test::black_box(time::get_time()))
}

#[bench]
fn slack_5ms(b: &mut test::Bencher) {
    let foo = SlackTimer::new(Duration::milliseconds(5));

    b.iter(|| test::black_box(foo.get_time()))
}

#[bench]
fn slack_50ms(b: &mut test::Bencher) {
    let foo = SlackTimer::new(Duration::milliseconds(50));

    b.iter(|| test::black_box(foo.get_time()))
}

#[bench]
fn slack_500ms(b: &mut test::Bencher) {
    let foo = SlackTimer::new(Duration::milliseconds(500));

    b.iter(|| test::black_box(foo.get_time()))
}

#[bench]
fn through_arc_50ms(b: &mut test::Bencher) {
    let foo = Arc::new(SlackTimer::new(Duration::milliseconds(50)));
    let inner = foo.clone();

    b.iter(|| test::black_box(inner.get_time()));
}
