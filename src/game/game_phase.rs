/// The action the game is currently doing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GamePhase {
    /// The game is being initialized.
    Init,

    /// The game is at the start of the frame.
    FrameStart,

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
    /// The game will exits.
    Exit,
}
