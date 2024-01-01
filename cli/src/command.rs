use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adjust the desk upwards. If specified, adjsut upwards in centimeters.
    Up {
        /// Height to change, in cm
        diff: Option<f32>,
    },

    /// Adjust the desk downwards. If specified, adjsut downwards in centimeters.
    Down {
        /// Height to change, in cm
        diff: Option<f32>,
    },

    /// Set the desk height to the specified centimeters. Not so accurate.
    Set {
        /// Height in cm
        height: f32,
    },

    /// Move to the position you saved for standing height.
    Standing,

    /// Move to the position you saved for sitting height.
    Sitting,

    /// Move to the position 1.
    Preset1,

    /// Move to the position 2.
    Preset2,

    /// Move to the position 3, alias for "standing" position.
    Preset3,

    /// Move to the position 4, alias for "sitting" position.
    Preset4,

    /// Query current height
    Query,
}
