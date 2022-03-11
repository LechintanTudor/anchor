use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShouldYield {
    No,
    Yes,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShouldRun {
    No,
    Yes,
}

#[derive(Clone, Copy)]
pub struct FpsLimiter {
    target_frame_duration: Duration,
    max_accumulator: Duration,
    old_time: Instant,
    accumulator: Duration,
}

impl FpsLimiter {
    pub fn new(target_fps: u32, max_updates_per_frame: u32) -> Self {
        let target_frame_duration = Duration::from_secs(1) / target_fps;

        Self {
            target_frame_duration,
            max_accumulator: max_updates_per_frame * target_frame_duration,
            old_time: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }

    pub fn begin(&mut self) -> ShouldYield {
        let current_time = Instant::now();
        let last_frame_time = current_time - self.old_time;

        self.old_time = current_time;
        self.accumulator += last_frame_time;

        if self.accumulator > self.max_accumulator {
            self.accumulator = self.max_accumulator;
        }

        if last_frame_time < self.target_frame_duration {
            ShouldYield::Yes
        } else {
            ShouldYield::No
        }
    }

    pub fn update(&mut self) -> ShouldRun {
        if self.accumulator >= self.target_frame_duration {
            self.accumulator -= self.target_frame_duration;
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }
}
