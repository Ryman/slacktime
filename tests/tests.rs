#![feature(macro_rules)]
extern crate slacktime;
extern crate time;
extern crate test;

macro_rules! tests(
    ($method:ident |$new:ident, $old:ident| $cmp:block) => (
        mod $method {
            use slacktime::SlackTimer;
            use std::time::Duration;
            use std::io::timer::sleep;
            use std::sync::{Arc, Future};

            fn t(d: Duration) {
                let foo = SlackTimer::new(d);
                //let cmp = |new, old| $cmp;
                range(0, 10u).fold(foo.$method(), |$old, _| {
                    println!("old: {}", $old)
                    sleep(d * 2); // bit of a buffer
                    let $new = foo.$method();
                    println!("new: {}", $new)
                    assert!($cmp, "{} was not greater than {}", $new, $old);

                    $new
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

                let futures = Vec::from_fn(100000, |_| {
                    let timer = shared.clone();
                    Future::spawn(proc() timer.$method())
                });

                for mut future in futures.into_iter() {
                    println!("{}", future.get());
                }
            }
        }
    )
)

tests!(get_time |new, old| { new > old })
tests!(now_utc |new, old| { new.to_timespec() > old.to_timespec() })
tests!(now |new, old| { new.to_timespec() > old.to_timespec() })
