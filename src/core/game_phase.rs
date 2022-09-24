/// The action the game is currently doing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GamePhase {
    /// The game is being initialized.
    Init,
    /// The game handles input.
    Input,
    /// The game is doing an update at the beginning of the frame.
    Update,
    /// The game is doing a fixed update.
    FixedUpdate,
    /// The game is doing an update before drawing.
    LateUpdate,
    /// The game is drawing.
    Draw,
    /// The game is being destroyed.
    Destroy,
}

impl GamePhase {
    /// Returns the associated error code.
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
