#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// How long before the deadline we switch from `thread::sleep` to a spin-loop.
/// 2 ms balances CPU burn against precision on most operating systems.
#[cfg(not(target_arch = "wasm32"))]
const SPIN_THRESHOLD: f64 = 0.002;

/// Number of recent frame times kept for the rolling average used by [`TimeManager::display_fps`].
const FPS_WINDOW: usize = 60;

/// Tracks frame timing and enforces an optional FPS cap.
///
/// Exposes two different FPS readings:
/// - [`fps`](Self::fps): instantaneous, recalculated every single frame.
/// - [`display_fps`](Self::display_fps): rolling average over the last [`FPS_WINDOW`] frames.
///
/// # Usage
/// ```ignore
/// time.begin_frame();
/// // ... render ...
/// time.end_frame();
/// time.every_seconds(1.0, |fps| window.set_title_fps(fps));
/// ```
#[derive(Debug)]
pub struct TimeManager {
    fps: f64,
    frame_target: f64,
    frame_start: Instant,
    frame_last: Instant,
    last_interval_tick: Instant,
    frame_times: [f64; FPS_WINDOW],
    frame_times_idx: usize,
    frame_times_filled: usize,
}

impl TimeManager {
    pub fn new() -> Self {
        let now: Instant = Instant::now();
        Self {
            fps: 0.0,
            frame_target: 0.0,
            frame_start: now,
            frame_last: now,
            last_interval_tick: now,
            frame_times: [0.0; FPS_WINDOW],
            frame_times_idx: 0,
            frame_times_filled: 0,
        }
    }

    /// Sets the FPS cap. Pass `0.0` (or any non-positive value) to uncap.
    pub fn set_target_fps(&mut self, target_fps: f64) {
        self.frame_target = if target_fps > 0.0 { 1.0 / target_fps } else { 0.0 };
    }

    /// Call at the **start** of every frame, before any rendering work.
    pub fn begin_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    /// Call at the **end** of every frame, after `present()`.
    ///
    /// Updates both the instantaneous and rolling-average FPS readings, then sleeps/spins to honour the cap.
    pub fn end_frame(&mut self) {
        let now: Instant = Instant::now();
        let delta: f64 = now.duration_since(self.frame_last).as_secs_f64();
        self.frame_last = now;

        if delta > 0.0 {
            self.fps = 1.0 / delta;
            self.push_frame_time(delta);
        }

        #[cfg(not(target_arch = "wasm32"))]
        self.enforce_cap();
    }

    /// Returns the time in seconds between the last two frames. Uses the **instantaneous** FPS
    pub fn delta_time(&self) -> f64 {
        if self.fps > 0.0 { 1.0 / self.fps } else { 0.0 }
    }

    /// Returns the **instantaneous** FPS — recalculated every single frame.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Returns a **smoothed** FPS reading averaged over the last [`FPS_WINDOW`] frames.
    pub fn display_fps(&self) -> f64 {
        if self.frame_times_filled == 0 {
            return 0.0;
        }

        let sum: f64 = self.frame_times[..self.frame_times_filled].iter().sum();
        let avg_delta: f64 = sum / self.frame_times_filled as f64;
        if avg_delta > 0.0 { 1.0 / avg_delta } else { 0.0 }
    }

    /// Invokes `callback` with the current **smoothed** FPS at most once per `interval` seconds.
    pub fn every_seconds<F: FnOnce(f64)>(&mut self, interval: f64, callback: F) {
        let now: Instant = Instant::now();
        if now.duration_since(self.last_interval_tick).as_secs_f64() >= interval {
            self.last_interval_tick = now;
            callback(self.display_fps());
        }
    }

    /// Pushes a new frame delta into the ring buffer, overwriting the oldest sample once the buffer is full.
    fn push_frame_time(&mut self, delta: f64) {
        self.frame_times[self.frame_times_idx] = delta;
        self.frame_times_idx = (self.frame_times_idx + 1) % FPS_WINDOW;

        if self.frame_times_filled < FPS_WINDOW {
            self.frame_times_filled += 1;
        }
    }

    /// Hybrid frame limiter: coarse `sleep` + precision spin-loop.
    #[cfg(not(target_arch = "wasm32"))]
    fn enforce_cap(&self) {
        if self.frame_target == 0.0 {
            return;
        }

        let elapsed: f64 = self.frame_start.elapsed().as_secs_f64();
        let remaining: f64 = self.frame_target - elapsed;

        if remaining <= 0.0 {
            return;
        }

        if remaining > SPIN_THRESHOLD {
            thread::sleep(Duration::from_secs_f64(remaining - SPIN_THRESHOLD));
        }

        while self.frame_start.elapsed().as_secs_f64() < self.frame_target {
            std::hint::spin_loop();
        }
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let tm: TimeManager = TimeManager::new();
        assert_eq!(tm.fps, 0.0);
        assert_eq!(tm.frame_target, 0.0);
        assert_eq!(tm.display_fps(), 0.0);
    }

    #[test]
    fn set_target_fps() {
        let mut tm: TimeManager = TimeManager::new();
        const EPS: f64 = 1e-9;

        tm.set_target_fps(60.0);
        assert!((tm.frame_target - 1.0 / 60.0).abs() < EPS);

        tm.set_target_fps(144.0);
        assert!((tm.frame_target - 1.0 / 144.0).abs() < EPS);

        tm.set_target_fps(0.0);
        assert_eq!(tm.frame_target, 0.0);

        tm.set_target_fps(-1.0);
        assert_eq!(tm.frame_target, 0.0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn fps_measurement() {
        let mut tm: TimeManager = TimeManager::new();
        tm.end_frame();
        thread::sleep(Duration::from_millis(16));
        tm.end_frame();
        assert!(tm.fps > 50.0 && tm.fps < 80.0, "fps={}", tm.fps);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn display_fps_smooths_spikes() {
        let mut tm: TimeManager = TimeManager::new();

        for _ in 0..10 {
            tm.end_frame();
            thread::sleep(Duration::from_millis(16));
        }

        tm.end_frame();
        thread::sleep(Duration::from_millis(200));
        tm.end_frame();

        assert!(tm.fps() < 15.0, "instant fps should reflect the spike, got {}", tm.fps());

        assert!(
            tm.display_fps() > 30.0,
            "rolling average should absorb a single spike, got {}",
            tm.display_fps()
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn display_fps_converges_to_stable_rate() {
        let mut tm: TimeManager = TimeManager::new();

        for _ in 0..FPS_WINDOW {
            tm.end_frame();
            thread::sleep(Duration::from_millis(10));
        }

        let display: f64 = tm.display_fps();
        assert!(
            display > 70.0 && display < 130.0,
            "display_fps should converge near 100, got {display}"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn interval_callback() {
        let mut tm: TimeManager = TimeManager::new();
        let mut fired: bool = false;

        tm.every_seconds(0.05, |_: f64| fired = true);
        assert!(!fired);

        thread::sleep(Duration::from_millis(60));
        tm.every_seconds(0.05, |_: f64| fired = true);
        assert!(fired);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn frame_cap_accuracy() {
        let mut tm: TimeManager = TimeManager::new();
        tm.set_target_fps(100.0);

        let start: Instant = Instant::now();
        tm.begin_frame();
        tm.end_frame();
        let elapsed: f64 = start.elapsed().as_secs_f64();

        assert!(elapsed >= 0.010, "limiter fired too early: {elapsed:.4}s");
        assert!(elapsed < 0.015, "limiter overslept: {elapsed:.4}s");
    }
}
