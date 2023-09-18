mod game_phase;

pub use self::game_phase::*;

use crate::game::Config;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct TimeConsts {
    pub frame_interval: Duration,
    pub fixed_update_interval: Duration,
    pub max_fixed_update_accumulator: Duration,
}

impl TimeConsts {
    pub fn new(config: &Config) -> Self {
        let one_second = Duration::from_secs(1);
        let frame_interval = one_second.div_f64(config.frames_per_second);
        let fixed_update_interval = one_second.div_f64(config.fixed_updates_per_second);

        let max_fixed_update_accumulator =
            fixed_update_interval.mul_f64(config.max_fixed_updates_per_frame);

        Self {
            frame_interval,
            fixed_update_interval,
            max_fixed_update_accumulator,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeContext {
    pub consts: TimeConsts,
    pub phase: GamePhase,
    pub frame_start: Instant,
    pub last_frame_duration: Duration,
    pub fixed_update_accumulator: Duration,
}

impl TimeContext {
    pub fn new(config: &Config) -> Self {
        let consts = TimeConsts::new(config);

        Self {
            phase: GamePhase::Init,
            frame_start: Instant::now(),
            last_frame_duration: consts.frame_interval,
            fixed_update_accumulator: Duration::ZERO,
            consts,
        }
    }

    pub fn start_frame(&mut self) {
        let now = Instant::now();
        self.last_frame_duration = now - self.frame_start;
        self.frame_start = now;

        self.fixed_update_accumulator += self.last_frame_duration;
        if self.fixed_update_accumulator > self.consts.max_fixed_update_accumulator {
            self.fixed_update_accumulator = self.consts.max_fixed_update_accumulator;
        }
    }

    pub fn fixed_update(&mut self) -> bool {
        if self.fixed_update_accumulator < self.consts.fixed_update_interval {
            return false;
        }

        self.fixed_update_accumulator -= self.consts.fixed_update_interval;
        true
    }

    pub fn frame_ended(&self) -> bool {
        Instant::now() - self.frame_start >= self.consts.frame_interval
    }

    pub fn fixed_delta_f32(&self) -> f32 {
        self.consts.fixed_update_interval.as_secs_f32()
    }

    pub fn variable_delta_f32(&self) -> f32 {
        self.last_frame_duration.as_secs_f32()
    }

    pub fn delta_f32(&self) -> f32 {
        if self.phase == GamePhase::FixedUpdate {
            self.fixed_delta_f32()
        } else {
            self.variable_delta_f32()
        }
    }
}

impl AsRef<TimeContext> for TimeContext {
    fn as_ref(&self) -> &TimeContext {
        self
    }
}
