#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(not(target_arch = "wasm32"))]
const MAX_SPIN_LOOP_DURATION: f64 = 0.002;

#[derive(Debug)]
pub struct TimeManager {
    frame_last_time: Instant,
    frame_start_time: Instant,
    frame_target_duration: f64,
    fps: f64,
}

// Do all FPS tracking for perfomance measurements and etc
impl TimeManager {
    pub fn new() -> Self {
        let now: Instant = Instant::now();
        Self {
            frame_last_time: now,
            frame_start_time: now,
            frame_target_duration: 0.0, // 0.0 -> unlimited
            fps: 0.0,
        }
    }

    // Get current FPS
    #[allow(dead_code)]
    pub fn get_fps(&self) -> f64 {
        self.fps
    }

    // Updates the target frame rate.
    //
    // If `target_fps` is 0, the limit is removed (unlimited FPS).
    // This automatically calculates the target duration in nanoseconds.
    //
    // # Arguments
    //
    // * `target_fps` - The desired frames per second (e.g., 60.0, 144.0). Pass 0.0 to unlock.
    pub fn set_target_fps(&mut self, target_fps: f64) {
        if target_fps > 0.0 {
            self.frame_target_duration = 1.0 / target_fps;
        } else {
            self.frame_target_duration = 0.0;
        }
    }

    // Call at the **start of the frame**
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
        let delta_frame_time: f64 = now.duration_since(self.frame_last_time).as_secs_f64();
        self.frame_last_time = now;

        if delta_frame_time > 0.0 {
            self.fps = 1.0 / delta_frame_time;
        }

        // FPS limit only Desktop
        // Hibrid approach, with sleep followed by a spin_loop in last 2 ms
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.frame_target_duration == 0.0 {
                return;
            }

            let elapsed: f64 = self.frame_start_time.elapsed().as_secs_f64();
            if elapsed < self.frame_target_duration {
                let remaining: f64 = self.frame_target_duration - elapsed;

                if remaining > MAX_SPIN_LOOP_DURATION {
                    thread::sleep(Duration::from_secs_f64(remaining - MAX_SPIN_LOOP_DURATION));
                }

                // Spin wait for 10 ms at maximum for the final precision (burns CPU but is accurate)
                while self.frame_start_time.elapsed().as_secs_f64() < self.frame_target_duration {
                    // distinct hint to the processor that we are spinning
                    std::hint::spin_loop();
                }
            }
        }
    }
}
