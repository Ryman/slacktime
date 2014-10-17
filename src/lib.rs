extern crate time;
use time::Timespec;
use std::time::Duration;
use std::io::timer::sleep;
use std::sync::Arc;
use std::sync::atomics::{AtomicPtr, Relaxed};
use std::comm::Disconnected;

/// A coarse timer updating on a background thread.
///
/// The sender channel is stored so whenever the SlackTimer
/// is dropped, the background thread will halt.
pub struct SlackTimer(Sender<()>, Arc<AtomicPtr<Timespec>>);

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
            let (ref mut a, ref mut b) = (time::get_time(), time::get_time());

            // Check if host process has killed the channel
            while rx.try_recv() != Err(Disconnected) {
                // Update one location at a time and then update the pointer
                *a = time::get_time();
                proc_timer.store(a, Relaxed);
                sleep(sleep_dur);
                *b = time::get_time();
                proc_timer.store(b, Relaxed);
                sleep(sleep_dur);
            }
        });

        // Wait for first assignment
        while timer.load(Relaxed).is_null() {}

        SlackTimer(tx, timer)
    }

    /// Returns the current time as a `Timespec` containing the
    /// seconds and nanoseconds since 1970-01-01T00:00:00Z.
    ///
    /// This is equivalent to a coarse form of `time::get_time`
    #[inline]
    pub fn get_time(&self) -> Timespec {
        let SlackTimer(_, ref time) = *self;
        unsafe { *time.load(Relaxed).clone() }
    }
}
