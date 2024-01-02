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

    /// Go to the preset position [possible values: standing/preset3, sitting/preset4, preset1,
    /// preset2]
    Go {
        /// Preset name
        #[clap(value_enum)]
        preset: Preset,
    },

    /// Set the desk height to the specified centimeters. Not so accurate.
    Set {
        /// Height to set, in cm.
        height: f32,
    },

    /// Query current height.
    Query,
}

impl From<Command> for flexispot_e7_controller_lib::Command {
    fn from(c: Command) -> Self {
        match c {
            Command::Up { diff } => flexispot_e7_controller_lib::Command::Up { diff },
            Command::Down { diff } => flexispot_e7_controller_lib::Command::Down { diff },
            Command::Go { preset } => flexispot_e7_controller_lib::Command::Go { preset },
            Command::Set { height } => flexispot_e7_controller_lib::Command::Set { height },
            Command::Query => flexispot_e7_controller_lib::Command::Query,
        }
    }
}
