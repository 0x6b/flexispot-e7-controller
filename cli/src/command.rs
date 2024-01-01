use clap::Subcommand;
use flexispot_e7_controller_lib::Preset;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adjust the desk upwards. If specified, adjsut upwards in centimeters. Not so accurate.
    Up {
        /// Height to change, in cm.
        diff: Option<f32>,
    },

    /// Adjust the desk downwards. If specified, adjsut downwards in centimeters. Not so accurate.
    Down {
        /// Height to change, in cm.
        diff: Option<f32>,
    },

    /// Set the desk height to the specified centimeters. Not so accurate.
    Set {
        /// Height to set, in cm.
        height: f32,
    },

    /// Go to the preset position [possible values: standing, sitting, preset1, preset2, preset3,
    /// preset4]
    Go {
        /// Preset name
        #[clap(value_enum)]
        preset: Preset,
    },

    /// Query current height.
    Query,
}
