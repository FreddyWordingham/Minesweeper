//! Compile time configuration.

/// Window MSAA samples.
pub const WINDOW_SAMPLES: u32 = 1;

/// Window background colour.
pub const WINDOW_CLEAR_COL: &str = "#444";

/// Window title.
pub const WINDOW_TITLE: &str = "Minesweeper";

/// Window size [pixels].
pub const WINDOW_RES: [f32; 2] = [800.0, 600.0];

/// Board colour.
pub const BOARD_COL: &str = "#A77";

/// Button colour.
pub const BUTTON_COL: &str = "#44C";

/// Button hover colour.
pub const BUTTON_HOVER_COL: &str = "#66C";

/// Button text colour.
pub const BUTTON_TEXT_COL: &str = "#EEE";

/// Camera zoom ratio (should be greater than zero and less than unity).
pub const CAMERA_ZOOM_SPEED: f32 = 0.5;

/// Camera pan rate (should be greater than zero).
pub const CAMERA_PAN_SPEED: f32 = 2.0;
