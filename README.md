# systrack
Track system resource usage over custom lapses.

## Usage examples

Create a tracker that records CPU usage.
```rust
let mut cpu_tracker = SystemTracker::new_cpu_tracker();
```
This creates a structure that keep track of CPU usage over the last minute (by default).
Interally, a background thread updates the historical information twice per second.

If you want a tracker to save historical data for a longer lapse, you should create the tracker using `new_cpu_tracker_with_capacity()`.


To get the historical data, you can call `fetch_usage()`.
```rust
for _ in 0..10 {
  println!("{:?}", cpu_tracker.fetch_usage());
  thread::sleep(Duration::from_millis(500));
}
```

Also, if you want the tracker to stop recording, you can call `stop()`.
```rust
cpu_tracker.stop();
for _ in 0..5 {
  println!("{:?}", cpu_tracker.fetch_usage());
  thread::sleep(Duration::from_millis(500));
}
```
Keep in mind that once the `stop()` method is called, the `fetch_usage()` method will always return `None`.

Memory usage tracker works almost identical. Check the examples dir for more.
