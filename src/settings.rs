//! Compile time configuration.

/// Window MSAA samples.
pub const WINDOW_SAMPLES: u32 = 1;

/// Window background colour.
pub const WINDOW_CLEAR_COL: &str = "#444";

/// Window title.
pub const WINDOW_TITLE: &str = "Minesweeper";

/// Window size [pixels].
pub const WINDOW_SIZE: [f32; 2] = [1200.0, 800.0];

/// Number pixels between tiles at a camera scale of 1.0.
pub const TILE_PADDING: f32 = 2.0;

/// Number pixels per tile at a camera scale of 1.0.
pub const TILE_SIZE: f32 = 50.0;

/// Map size [tiles].
pub const MAP_RES: [i16; 2] = [200, 200];

/// Map size [pixels].
pub const MAP_SIZE: [f32; 2] = [MAP_RES[0] as f32 * TILE_SIZE, MAP_RES[1] as f32 * TILE_SIZE];

/// Number of bombs.
pub const NUM_BOMBS: u16 = 1000;

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

/// Camera minimum zoom.
pub const CAMERA_MIN_ZOOM: f32 = 0.125;

/// Camera maximum zoom.
pub const CAMERA_MAX_ZOOM: f32 = 4.0;
