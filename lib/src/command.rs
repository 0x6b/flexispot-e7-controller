use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Command {
    /// Adjust the desk upwards
    Up {
        /// Height to change, in cm.
        diff: Option<f32>,
    },

    /// Adjust the desk downwards
    Down {
        /// Height to change, in cm.
        diff: Option<f32>,
    },

    /// Go to the preset position [possible values: standing, sitting, preset1, preset2, preset3,
    /// preset4]
    Go {
        /// Preset name
        preset: Preset,
    },

    /// Set the desk height to the specified centimeters. Not so accurate.
    Set {
        /// Height to set, in cm.
        height: f32,
    },

    /// Wake Up (no-op)
    WakeUp,

    /// Memory button
    Memory,

    /// Query current height.
    Query,
}

pub type CommandSequence = [u8; 8];

impl From<&Command> for CommandSequence {
    fn from(c: &Command) -> Self {
        use Command::*;
        use Preset::*;
        match c {
            Up { .. } => [0x9b, 0x06, 0x02, 0x01, 0x00, 0xfc, 0xa0, 0x9d],
            Down { .. } => [0x9b, 0x06, 0x02, 0x02, 0x00, 0x0c, 0xa0, 0x9d],
            Go { preset } => match preset {
                Standing | Preset3 => [0x9b, 0x06, 0x02, 0x10, 0x00, 0xac, 0xac, 0x9d],
                Sitting | Preset4 => [0x9b, 0x06, 0x02, 0x00, 0x01, 0xac, 0x60, 0x9d],
                Preset1 => [0x9b, 0x06, 0x02, 0x04, 0x00, 0xac, 0xa3, 0x9d],
                Preset2 => [0x9b, 0x06, 0x02, 0x08, 0x00, 0xac, 0xa6, 0x9d],
            },
            WakeUp => [0x9b, 0x06, 0x02, 0x00, 0x00, 0x6c, 0xa1, 0x9d],
            Memory => [0x9b, 0x06, 0x02, 0x20, 0x00, 0xac, 0xb8, 0x9d],
            Set { .. } => unreachable!("Set variant should not be used to command the controller"),
            Query => unreachable!("Query variant should not be used to command the controller"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preset {
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
}

impl From<String> for Preset {
    fn from(s: String) -> Self {
        use Preset::*;
        match s.to_lowercase().as_str() {
            s if s.starts_with("st") => Standing,
            s if s.starts_with("si") => Sitting,
            s if s.starts_with('p') && s.ends_with('1') => Preset1,
            s if s.starts_with('p') && s.ends_with('2') => Preset2,
            s if s.starts_with('p') && s.ends_with('3') => Preset3,
            s if s.starts_with('p') && s.ends_with('4') => Preset4,
            _ => Standing,
        }
    }
}
