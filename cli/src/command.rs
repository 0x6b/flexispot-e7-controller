use clap::{Subcommand, ValueEnum};

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

    /// Go to the preset position [default: standing] [possible values: standing, sitting, preset1, preset2, preset3, preset4]
    Go {
        /// Preset name
        #[clap(value_enum)]
        preset: Preset,
    },

    /// Query current height.
    Query,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Preset {
    /// Standing height
    Standing,

    /// Sitting height
    Sitting,

    /// Position 1
    Preset1,

    /// Position 2
    Preset2,

    /// Position 3, alias for Standing
    Preset3,

    /// Position 4, alias for Sitting
    Preset4,
}

impl From<String> for Preset {
    fn from(s: String) -> Self {
        use Preset::*;
        match s.to_lowercase().as_str() {
            "standing" => Standing,
            "sitting" => Sitting,
            "preset1" => Preset1,
            "preset2" => Preset2,
            "preset3" => Preset3,
            "preset4" => Preset4,
            _ => Standing,
        }
    }
}
