/// Info contains information about the block.
pub struct Info {
    pub height: u64,
    pub time: u64,
}

impl Info {
    /// Creates a new info.
    pub fn new(height: u64, time: u64) -> Self {
        Self { height, time }
    }
    /// Returns the height of the block.
    pub fn height(&self) -> u64 {
        self.height
    }
    /// Returns the time of the block.
    pub fn time(&self) -> u64 {
        self.time
    }
}
