#[derive(Debug, Clone)]
pub enum Command {
    /// Adjust the desk upwards
    Up,

    /// Adjust the desk downwards
    Down,

    /// The position of the standing height saved
    Standing,

    /// The position of the sitting height saved
    Sitting,

    /// Position 1, first height position saved
    Preset1,

    /// Position 2, second height position saved
    Preset2,

    /// Position 3, alias for "standing" position
    Preset3,

    /// Position 4, alias for "sitting" position
    Preset4,

    /// Wake Up (no-op)
    #[allow(dead_code)]
    WakeUp,

    /// Memory button
    Memory,
}

impl Command {
    pub fn command(&self) -> [u8; 8] {
        use Command::*;
        match *self {
            Up => [0x9b, 0x06, 0x02, 0x01, 0x00, 0xfc, 0xa0, 0x9d],
            Down => [0x9b, 0x06, 0x02, 0x02, 0x00, 0x0c, 0xa0, 0x9d],
            Standing | Preset3 => [0x9b, 0x06, 0x02, 0x10, 0x00, 0xac, 0xac, 0x9d],
            Sitting | Preset4 => [0x9b, 0x06, 0x02, 0x00, 0x01, 0xac, 0x60, 0x9d],
            Preset1 => [0x9b, 0x06, 0x02, 0x04, 0x00, 0xac, 0xa3, 0x9d],
            Preset2 => [0x9b, 0x06, 0x02, 0x08, 0x00, 0xac, 0xa6, 0x9d],
            WakeUp => [0x9b, 0x06, 0x02, 0x00, 0x00, 0x6c, 0xa1, 0x9d],
            Memory => [0x9b, 0x06, 0x02, 0x20, 0x00, 0xac, 0xb8, 0x9d],
        }
    }
}
