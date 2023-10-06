use cpu::{CpuTracker, TrackerCapacity};

pub mod cpu;
pub struct SystemTracker;

impl SystemTracker{
    /// Create a tracker that records each system's cpu usage over 1 minute.
    /// ```
    /// let mut cpu_tracker = SystemTracker::new_cpu_tracker();
    /// ```
    pub fn new_cpu_tracker() -> CpuTracker{
        CpuTracker::default()
    }

    /// Create a tracker that records each system's cpu usage over a specified time.
    /// ```
    /// let mut cpu_tracker =
    ///     SystemTracker::new_cpu_tracker_with_capacity(TrackerCapacity::FIVE);
    /// let mut cpu_tracker =
    ///     SystemTracker::new_cpu_tracker_with_capacity(TrackerCapacity::CUSTOM(20));
    /// ```
    pub fn new_cpu_tracker_with_capacity(minutes: TrackerCapacity) -> CpuTracker{
        CpuTracker::with_capacity(minutes)
    }

    pub fn new_memory_tracker(){

    }
}