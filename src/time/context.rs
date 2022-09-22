use crate::time::TimeConfig;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct TimeContext {
    frame_interval: Duration,
    fixed_update_interval: Duration,
    start_time: Instant,
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
            start_time: Instant::now(),
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

    pub fn set_target_fps(&mut self, target_fps: f64) {
        self.frame_interval = Duration::from_secs(1).div_f64(target_fps)
    }

    pub fn set_target_tps(&mut self, target_tps: f64) {
        self.fixed_update_interval = Duration::from_secs(1).div_f64(target_tps)
    }

    pub fn since_start(&self) -> Duration {
        Instant::now() - self.start_time
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
