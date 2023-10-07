use std::{time::Duration, thread};

use systrack::{SystemTracker, TrackerCapacity};

fn main(){
    //create a cpu tracker with default capacity
    let mut cpu_tracker = SystemTracker::new_cpu_tracker();
    for _ in 0..10 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    cpu_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }

    //create a cpu tracker with 5 minutes capacity
    let mut cpu_tracker = SystemTracker::new_cpu_tracker_with_capacity(TrackerCapacity::FIVE);
    for _ in 0..10 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    cpu_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }

    //create a cpu tracker with 120 minutes capacity
    let mut cpu_tracker = SystemTracker::new_cpu_tracker_with_capacity(TrackerCapacity::CUSTOM(20));
    for _ in 0..10 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    cpu_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", cpu_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }

}