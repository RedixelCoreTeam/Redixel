use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FpsTracker {
    frame_last_time: Instant,
    frame_start_time: Instant,
    frame_target_duration: u128,
    pub fps: f64,
}

// Do all FPS tracking for perfomance measurements and etc
impl FpsTracker {
    pub fn new() -> Self {
        let now: Instant = Instant::now();
        Self {
            frame_last_time: now,
            frame_start_time: now,
            frame_target_duration: 0, // 0 -> unlimited
            fps: 0.0,
        }
    }

    // Updates the target frame rate.
    //
    // If `target_fps` is 0, the limit is removed (unlimited FPS).
    // This automatically calculates the target duration in nanoseconds.
    //
    // # Arguments
    //
    // * `target_fps` - The desired frames per second (e.g., 60, 144). Pass 0 to unlock.
    pub fn set_target_fps(&mut self, target_fps: u32) {
        if target_fps > 0 {
            self.frame_target_duration = 1_000_000_000 / target_fps as u128;
        } else {
            self.frame_target_duration = 0;
        }
    }

    // Call at the **start of the frame**
    //
    pub fn begin_frame(&mut self) {
        self.frame_start_time = Instant::now();
    }

    // Call this at the very **end** of your game loop (after `present()`).
    //
    // Details:
    // - Updates the `fps` every frame
    // - Sleeps/Waits to limit the target FPS (if set).
    pub fn end_frame(&mut self) {
        let now: Instant = Instant::now();
        let delta_frame_time: u128 = now.duration_since(self.frame_last_time).as_nanos();
        self.frame_last_time = now;

        if delta_frame_time > 0 {
            self.fps = 1_000_000_000.0 / (delta_frame_time as f64);
        }

        if self.frame_target_duration == 0 {
            return;
        }

        // FPS limit
        let elapsed: u128 = self.frame_start_time.elapsed().as_nanos();
        if elapsed < self.frame_target_duration {
            let remaining: u128 = self.frame_target_duration - elapsed;

            if remaining > 2_000_000 {
                thread::sleep(Duration::from_nanos((remaining - 1_500_000) as u64));
            }

            // Spin wait for 1.5 ms at maximum for the final precision (burns CPU but is accurate)
            while self.frame_start_time.elapsed().as_nanos() < self.frame_target_duration {
                // distinct hint to the processor that we are spinning
                std::hint::spin_loop();
            }
        }
    }
}
