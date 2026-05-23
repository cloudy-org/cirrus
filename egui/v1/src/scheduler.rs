use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};

#[derive(Clone)]
struct InnerScheduler<T> {
    delay: Duration,
    callback: Arc<Mutex<dyn FnMut() -> T + Send>>,
    time_scheduled: Instant,
}

/// A neat way to handle scheduling events for later in a egui update loop.
/// 
/// ```rust
/// use std::time::Duration;
/// use cirrus_egui::scheduler::Scheduler;
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
#[derive(Clone)]
pub struct Scheduler<T = ()> {
    inner: Option<InnerScheduler<T>>,
    pub done: bool
}

impl<T> Scheduler<T> {
    pub const UNSET: Self = Self { inner: None, done: true };

    pub fn new(callback: impl FnMut() -> T + Send + 'static, delay: Duration) -> Self {
        Self {
            inner: Some(
                InnerScheduler {
                    delay,
                    callback: Arc::new(Mutex::new(callback)),
                    time_scheduled: Instant::now(),
                }
            ),
            done: false
        }
    }

    pub fn elapsed(&self) -> Duration {
        match &self.inner {
            Some(scheduler) => scheduler.time_scheduled.elapsed(),
            None => Duration::default(),
        }
    }

    /// Checks if the scheduled time has passed. You need to place this in your update loop.
    /// 
    /// This function only returns 'Some()' ONCE, and that is when the 
    /// callback is called and enough time has elapsed since the set duration / delay.
    pub fn update(&mut self) -> Option<T> {
        if self.done == true {
            return None;
        }

        if let Some(inner) = self.inner.as_mut() {
            if inner.time_scheduled.elapsed() >= inner.delay {

                if let Ok(mut callback) = inner.callback.lock() {
                    let return_value = (callback)();
                    self.done = true;

                    return Some(return_value);
                }
            }
        }

        None
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::UNSET
    }
}