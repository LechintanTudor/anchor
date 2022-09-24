#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GamePhase {
    Init,
    Input,
    Update,
    FixedUpdate,
    LateUpdate,
    Draw,
    Destroy,
}

impl GamePhase {
    pub const fn error_exit_code(&self) -> i32 {
        match self {
            Self::Init => 1,
            Self::Input => 2,
            Self::Update => 3,
            Self::FixedUpdate => 4,
            Self::LateUpdate => 5,
            Self::Draw => 6,
            Self::Destroy => 7,
        }
    }
}
