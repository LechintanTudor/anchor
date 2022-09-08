#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FramePhase {
    Input,
    Update,
    FixedUpdate,
    LateUpdate,
    Draw,
}

impl FramePhase {
    pub const fn error_exit_code(&self) -> i32 {
        match self {
            Self::Input => 1,
            Self::Update => 1,
            Self::FixedUpdate => 2,
            Self::LateUpdate => 3,
            Self::Draw => 4,
        }
    }
}
