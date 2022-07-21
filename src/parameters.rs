//! Application input.

use serde::Deserialize;

/// Input parameters.
#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Window title.
    pub window_title: String,
    /// Window clear colour.
    pub window_clear_col: String,
    /// Window resolution.
    pub window_res: [f32; 2],
}
