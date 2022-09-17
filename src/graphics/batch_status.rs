#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BatchStatus {
    /// Batch is empty
    #[default]
    Empty,
    /// Batch is not empty and is ready to upload data to the GPU
    NonEmpty,
    /// Batch is ready to draw
    Ready,
}
