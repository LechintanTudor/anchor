#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamePhase {
    Init,
    Update,
    FixedUpdate,
    LateUpdate,
    Draw,
    Input,
    Exit,
}
