use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    target_frame_duration: Duration,
    target_fixed_update_duration: Duration,
    max_frame_time_accumulator: Duration,
    frame_start_time: Instant,
    frame_time_accumulator: Duration,
}

impl Timer {
    pub fn new(target_frames_per_second: f64, target_fixed_updates_per_second: f64) -> Self {
        let target_frame_duration = Duration::from_secs(1).div_f64(target_frames_per_second);

        let target_fixed_update_duration =
            Duration::from_secs(1).div_f64(target_fixed_updates_per_second);

        let target_fixed_updates_per_frame =
            target_fixed_updates_per_second / target_frames_per_second;

        let max_frame_time_accumulator =
            target_fixed_update_duration.mul_f64(target_fixed_updates_per_frame * 2.0);

        Timer {
            target_frame_duration,
            target_fixed_update_duration,
            max_frame_time_accumulator,
            frame_start_time: Instant::now(),
            frame_time_accumulator: Duration::ZERO,
        }
    }

    pub fn start_frame(&mut self) {
        let current_time = Instant::now();
        let frame_time = current_time - self.frame_start_time;
        self.frame_start_time = current_time;

        self.frame_time_accumulator += frame_time;
        if self.frame_time_accumulator > self.max_frame_time_accumulator {
            self.frame_time_accumulator = self.max_frame_time_accumulator;
        }
    }

    pub fn fixed_update(&mut self) -> bool {
        if self.frame_time_accumulator >= self.target_fixed_update_duration {
            self.frame_time_accumulator -= self.target_fixed_update_duration;
            true
        } else {
            false
        }
    }

    pub fn end_frame(&self) -> bool {
        Instant::now() - self.frame_start_time >= self.target_frame_duration
    }
}
