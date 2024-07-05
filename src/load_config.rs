use std::fs::read_to_string;
use std::str::FromStr;

use log::{info, warn};

use crate::defaults::{DEFAULT_DURATION, DEFAULT_TRIGGERS};

// Loads the configuration. Returns a default configuration if it fails at any point
// Default configuration: Empty set of triggers, 1 second dark
pub fn load_config(path: &str) -> (Vec<String>, f32) {
    fn default_triggers() -> Vec<String> {
        DEFAULT_TRIGGERS
            .iter()
            .clone()
            .map(|x| (**x).to_string())
            .collect()
    }
    let default_duration: f32 = DEFAULT_DURATION;
    match read_to_string(path) {
        Ok(s) => {
            let lines: Vec<String> = s.split("\n").map(|x| x.trim().into()).collect();
            let blind_len = {
                if lines.len() >= 2 {
                    match FromStr::from_str(&lines[1]) {
                        Ok(f) => f,
                        Err(_) => {
                            warn!(
                                "Failed to parse blind duration, defaulting to {}",
                                default_duration
                            );
                            default_duration
                        }
                    }
                } else {
                    info!(
                        "Blind duration left blank, defaulting to {}",
                        default_duration
                    );
                    default_duration
                }
            };
            let blinders = {
                if lines.len() >= 1 {
                    lines[0]
                        .split(",")
                        .map(|x| x.trim().to_lowercase().into())
                        .collect::<Vec<String>>()
                } else {
                    info!(
                        "Triggers left blank, defaulting to {:?}",
                        default_triggers()
                    );
                    default_triggers()
                }
            };
            (blinders, blind_len)
        }
        Err(_) => {
            warn!(
                "No config found at {}. Using default config with triggers {:?} and duration {}",
                path,
                default_triggers(),
                default_duration
            );
            (default_triggers(), 1.0)
        }
    }
}
