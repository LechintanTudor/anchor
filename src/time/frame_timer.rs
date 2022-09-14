use std::time::{Duration, Instant};

#[derive(Debug)]
struct FrameTimerConsts {
    target_frame_interval: Duration,
    target_fixed_update_interval: Duration,
}

#[derive(Debug)]
pub(crate) struct FrameTimer {
    consts: FrameTimerConsts,
    frame_start_time: Instant,
    last_frame_duration: Duration,
    frame_duration_accumulator: Duration,
}

impl FrameTimer {
    pub fn new(target_frames_per_second: f64, target_fixed_updates_per_second: f64) -> Self {
        let target_frame_interval = Duration::from_secs(1).div_f64(target_frames_per_second);

        let target_fixed_update_interval =
            Duration::from_secs(1).div_f64(target_fixed_updates_per_second);

        FrameTimer {
            consts: FrameTimerConsts { target_frame_interval, target_fixed_update_interval },
            frame_start_time: Instant::now(),
            last_frame_duration: target_frame_interval,
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
        if self.frame_duration_accumulator >= self.consts.target_fixed_update_interval {
            self.frame_duration_accumulator -= self.consts.target_fixed_update_interval;
            true
        } else {
            false
        }
    }

    pub fn end_frame(&self) -> bool {
        Instant::now() - self.frame_start_time >= self.consts.target_frame_interval
    }

    #[inline]
    pub fn delta(&self) -> Duration {
        self.last_frame_duration
    }

    #[inline]
    pub fn fixed_delta(&self) -> Duration {
        self.consts.target_fixed_update_interval
    }

    #[inline]
    pub fn frame_duration_accumulator(&self) -> Duration {
        self.frame_duration_accumulator
    }
}
