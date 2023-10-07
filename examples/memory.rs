use std::{thread, time::Duration};

use systrack::{SystemTracker, TrackerCapacity};

fn main(){
    //create a memory tracker with default capacity
    let mut mem_tracker = SystemTracker::new_mem_tracker();
    for _ in 0..10 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    mem_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }

    //create a memory tracker with 5 minutes capacity
    let mut mem_tracker = SystemTracker::new_mem_tracker_with_capacity(TrackerCapacity::FIVE);
    for _ in 0..10 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    mem_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }

    //create a memory tracker with custom (20 min) capacity
    let mut mem_tracker = SystemTracker::new_mem_tracker_with_capacity(TrackerCapacity::CUSTOM(20));
    for _ in 0..10 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    mem_tracker.stop();
    for _ in 0..5 {
        println!("{:?}", mem_tracker.fetch_usage());
        thread::sleep(Duration::from_millis(500));
    }
    

}