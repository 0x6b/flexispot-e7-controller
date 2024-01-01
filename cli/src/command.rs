use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adjust the desk upwards
    Up {
        /// Height to change, in cm
        diff: Option<f32>,
    },

    /// Adjust the desk downwards
    Down {
        /// Height to change, in cm
        diff: Option<f32>,
    },

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

    /// Query current height
    Query,

    /// Set height
    Set {
        /// Height to change, in cm
        height: f32,
    },
}
