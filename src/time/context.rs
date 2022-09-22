use crate::time::TimeConfig;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct FrameTimerConsts {}

#[derive(Debug)]
pub(crate) struct TimeContext {
    frame_interval: Duration,
    fixed_update_interval: Duration,
    frame_start_time: Instant,
    last_frame_duration: Duration,
    frame_duration_accumulator: Duration,
}

impl TimeContext {
    pub fn new(config: TimeConfig) -> Self {
        let frame_interval = config.frame_interval();
        let fixed_update_interval = config.fixed_update_interval();

        Self {
            frame_interval,
            fixed_update_interval,
            frame_start_time: Instant::now(),
            last_frame_duration: frame_interval,
            frame_duration_accumulator: Duration::ZERO,
        }
    }

    pub fn start_frame(&mut self) {
        let current_time = Instant::now();
        self.last_frame_duration = current_time - self.frame_start_time;
        self.frame_start_time = current_time;

        self.frame_duration_accumulator += self.last_frame_duration;
    }

    pub fn fixed_update(&mut self) -> bool {
        if self.frame_duration_accumulator >= self.fixed_update_interval {
            self.frame_duration_accumulator -= self.fixed_update_interval;
            true
        } else {
            false
        }
    }

    pub fn end_frame(&self) -> bool {
        Instant::now() - self.frame_start_time >= self.frame_interval
    }

    #[inline]
    pub fn delta(&self) -> Duration {
        self.last_frame_duration
    }

    #[inline]
    pub fn fixed_delta(&self) -> Duration {
        self.fixed_update_interval
    }

    #[inline]
    pub fn frame_duration_accumulator(&self) -> Duration {
        self.frame_duration_accumulator
    }
}
