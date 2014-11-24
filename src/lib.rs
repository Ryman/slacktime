extern crate time;
use time::{Timespec, Tm};
use std::time::Duration;
use std::io::timer::sleep;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Relaxed};
use std::comm::Disconnected;
use std::cell::UnsafeCell;
use std::mem;

/// A coarse timer updating on a background thread.
///
/// The sender channel is stored so whenever the SlackTimer
/// is dropped, the background thread will halt.
pub struct SlackTimer(UnsafeCell<Sender<()>>, Arc<AtomicPtr<CachedTimes>>);

struct CachedTimes {
    get_time: Timespec,
    now: Tm,
    now_utc: Tm,
}

impl SlackTimer {
    /// Creates a new timer with a given period.
    ///
    /// # Important
    /// The granularity provided by the period should not be depended on
    /// being accurate, the tests try to ensure that each time given is
    /// within 2 sleep durations.
    ///
    /// It's best to consider it as specifying a ballpack of how granular
    /// you'd like to be.
    pub fn new(sleep_dur: Duration) -> SlackTimer {
        let timer = Arc::new(AtomicPtr::new(0 as *mut _));
        let proc_timer = timer.clone();
        let (tx, rx) = channel();

        spawn(proc() {
            let (ref mut a, ref mut b) = unsafe {
                (mem::uninitialized::<CachedTimes>(),
                 mem::uninitialized::<CachedTimes>())
            };

            // Check if host process has killed the channel
            while rx.try_recv() != Err(Disconnected) {
                // Update one location at a time and then update the pointer
                *a = CachedTimes {
                    get_time: time::get_time(),
                    now: time::now(),
                    now_utc: time::now_utc(),
                };
                proc_timer.store(a, Relaxed);
                sleep(sleep_dur);

                *b = CachedTimes {
                    get_time: time::get_time(),
                    now: time::now(),
                    now_utc: time::now_utc(),
                };
                proc_timer.store(b, Relaxed);
                sleep(sleep_dur);
            }
        });

        // Wait for first assignment
        while timer.load(Relaxed).is_null() {}

        SlackTimer(UnsafeCell::new(tx), timer)
    }

    /// Returns the current time as a `Timespec` containing the
    /// seconds and nanoseconds since 1970-01-01T00:00:00Z.
    ///
    /// This is equivalent to a coarse form of `time::get_time`
    #[inline]
    pub fn get_time(&self) -> Timespec {
        let SlackTimer(_, ref time) = *self;
        unsafe { (*time.load(Relaxed)).get_time.clone() }
    }

    /// Returns the current time in UTC
    ///
    /// This is equivalent to a coarse form of `time::now_utc`
    #[inline]
    pub fn now_utc(&self) -> Tm {
        let SlackTimer(_, ref time) = *self;
        unsafe { (*time.load(Relaxed)).now_utc.clone() }
    }

    /// Returns the current time in the local timezone
    ///
    /// This is equivalent to a coarse form of `time::now`
    #[inline]
    pub fn now(&self) -> Tm {
        let SlackTimer(_, ref time) = *self;
        unsafe { (*time.load(Relaxed)).now.clone() }
    }
}
