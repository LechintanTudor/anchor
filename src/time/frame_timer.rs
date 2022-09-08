use std::time::{Duration, Instant};

#[derive(Debug)]
struct FrameTimerConsts {
    target_frame_duration: Duration,
    target_fixed_update_duration: Duration,
    max_frame_duration_accumulator: Duration,
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
        let target_frame_duration = Duration::from_secs(1).div_f64(target_frames_per_second);

        let target_fixed_update_duration =
            Duration::from_secs(1).div_f64(target_fixed_updates_per_second);

        let target_fixed_updates_per_frame =
            target_fixed_updates_per_second / target_frames_per_second;

        let max_frame_duration_accumulator =
            target_fixed_update_duration.mul_f64(target_fixed_updates_per_frame * 2.0);

        FrameTimer {
            consts: FrameTimerConsts {
                target_frame_duration,
                target_fixed_update_duration,
                max_frame_duration_accumulator,
            },
            frame_start_time: Instant::now(),
            last_frame_duration: target_frame_duration,
            frame_duration_accumulator: Duration::ZERO,
        }
    }

    pub fn start_frame(&mut self) {
        let current_time = Instant::now();
        self.last_frame_duration = current_time - self.frame_start_time;
        self.frame_start_time = current_time;

        self.frame_duration_accumulator += self.last_frame_duration;
        if self.frame_duration_accumulator > self.consts.max_frame_duration_accumulator {
            self.frame_duration_accumulator = self.consts.max_frame_duration_accumulator;
        }
    }

    pub fn fixed_update(&mut self) -> bool {
        if self.frame_duration_accumulator >= self.consts.target_fixed_update_duration {
            self.frame_duration_accumulator -= self.consts.target_fixed_update_duration;
            true
        } else {
            false
        }
    }

    pub fn end_frame(&self) -> bool {
        Instant::now() - self.frame_start_time >= self.consts.target_frame_duration
    }

    #[inline]
    pub fn delta(&self) -> Duration {
        self.last_frame_duration
    }

    #[inline]
    pub fn fixed_delta(&self) -> Duration {
        self.consts.target_fixed_update_duration
    }
}
