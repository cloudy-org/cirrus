use std::time::{Duration, Instant};

/// A neat way to handle scheduling events for later in a egui update loop.
/// 
/// ```rust
/// use std::time::Duration;
/// use cirrus_egui::v1::scheduler::Scheduler;
/// 
/// struct UwUBroadcaster {
///     uwu_schedule: Scheduler
/// }
/// 
/// impl UwUBroadcaster {
///     pub fn new() -> Self {
///         Self {
///             uwu_schedule: Scheduler::new(
///                 || {
///                     // "UwU!" will be printed in the console in 3 seconds time.
///                     println!("UwU!");
///                 },
///                 Duration::from_secs(3)
///             )
///         }
///     }
/// 
///     pub fn update(&mut self) {
///         if self.uwu_schedule.update().is_some() {
///             // ".update()" will ONLY return "Some()" 
///             // ONCE, and that is when the callback is called.
///             println!("Callback has been executed!");
///         }
///     }
/// }
/// ```
pub struct Scheduler<T = ()> {
    delay: Duration,
    callback: Box<dyn FnMut() -> T>,
    time_scheduled: Instant,
    pub done: bool
}

impl<T> Scheduler<T> {
    pub fn new(callback: impl FnMut() -> T + 'static, delay: Duration) -> Self {
        Self {
            delay,
            callback: Box::new(callback),
            time_scheduled: Instant::now(),
            done: false
        }
    }

    /// Returns a copy of the scheduler but reset.
    // pub fn reset(self) -> Self {
    //     Self {
    //         delay: self.delay,
    //         callback: self.callback,
    //         time_scheduled: Instant::now(),
    //         done: false
    //     }
    // }

    pub fn update(&mut self) -> Option<T> {
        if self.done == true {
            return None;
        }

        if self.time_scheduled.elapsed() >= self.delay {
            let return_value = (self.callback)();
            self.done = true;

            return Some(return_value);
        }

        None
    }
}