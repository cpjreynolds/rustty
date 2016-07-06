//! Box-drawing, shade and other terminal graphic characters.
//!
//! # Abbreviations
//!
//! The constants defined below are named according to the unicode specification, and the following
//! abbreviations are used.
//!
//! - D - Dark
//! - H - Heavy
//! - M - Medium
//! - L - Light
//!
//! - DN - Down
//! - UP - Upper
//! - LR - Lower
//! - RT - Right
//! - LT - Left
//!
//! - HORIZ - Horizontal
//! - VERT - Vertical
//!
//! - QUAD - Quadrant/Quadruple
//! - TRIP - Triple
//! - DIAG - Diagonal
//!
//! - BOX - Box drawings
//!
//! - x/y - X_Y

// Block clements.
pub const UP_1_2_BLOCK: char = '\u{2580}';
pub const LR_1_8_BLOCK: char = '\u{2581}';
pub const LR_1_4_BLOCK: char = '\u{2582}';
pub const LR_3_8_BLOCK: char = '\u{2583}';
pub const LR_1_2_BLOCK: char = '\u{2584}';
pub const LR_5_8_BLOCK: char = '\u{2585}';
pub const LR_3_4_BLOCK: char = '\u{2586}';
pub const LR_7_8_BLOCK: char = '\u{2587}';
pub const FULL_BLOCK: char = '\u{2588}';
pub const LT_7_8_BLOCK: char = '\u{2589}';
pub const LT_3_4_BLOCK: char = '\u{258A}';
pub const LT_5_8_BLOCK: char = '\u{258B}';
pub const LT_1_2_BLOCK: char = '\u{258C}';
pub const LT_3_8_BLOCK: char = '\u{258D}';
pub const LT_1_4_BLOCK: char = '\u{258E}';
pub const LT_1_8_BLOCK: char = '\u{258F}';
pub const RT_1_2_BLOCK: char = '\u{2590}';

// Shade characters.
pub const L_SHADE: char = '\u{2591}';
pub const M_SHADE: char = '\u{2592}';
pub const D_SHADE: char = '\u{2593}';

// Block elements.
pub const UP_1_8_BLOCK: char = '\u{2594}';
pub const RT_1_8_BLOCK: char = '\u{2595}';

// Terminal graphic characters.
pub const QUAD_LR_LT: char = '\u{2596}';
pub const QUAD_LR_RT: char = '\u{2597}';
pub const QUAD_UP_LT: char = '\u{2598}';
pub const QUAD_UP_LT_LR_LT_LR_RT: char = '\u{2599}';
pub const QUAD_UP_LT_LR_RT: char = '\u{259A}';
pub const QUAD_UP_LT_UP_RT_LR_LT: char = '\u{259B}';
pub const QUAD_UP_LT_UP_RT_LR_RT: char = '\u{259C}';
pub const QUAD_UP_RT: char = '\u{259D}';
pub const QUAD_UP_RT_LR_LT: char = '\u{259E}';
pub const QUAD_UP_RT_LR_LT_LR_RT: char = '\u{259F}';

// Light and heavy solid lines.
pub const BOX_L_HORIZ: char = '\u{2500}';
pub const BOX_H_HORIZ: char = '\u{2501}';
pub const BOX_L_VERT: char = '\u{2502}';
pub const BOX_H_VERT: char = '\u{2503}';

// Light and heavy dashed lines.
pub const BOX_L_TRIP_DASH_HORIZ: char = '\u{2504}';
pub const BOX_H_TRIP_DASH_HORIZ: char = '\u{2505}';
pub const BOX_L_TRIP_DASH_VERT: char = '\u{2506}';
pub const BOX_H_TRIP_DASH_VERT: char = '\u{2507}';
pub const BOX_L_QUAD_DASH_HORIZ: char = '\u{2508}';
pub const BOX_H_QUAD_DASH_HORIZ: char = '\u{2509}';
pub const BOX_L_QUAD_DASH_VERT: char = '\u{250A}';
pub const BOX_H_QUAD_DASH_VERT: char = '\u{250B}';

// Light and heavy line box components.
pub const BOX_L_DN_RT: char = '\u{250C}';
pub const BOX_DN_L_RT_H: char = '\u{250D}';
pub const BOX_DN_H_RT_L: char = '\u{250E}';
pub const BOX_H_DN_RT: char = '\u{250F}';

pub const BOX_L_DN_LT: char = '\u{2510}';
pub const BOX_DN_L_LT_H: char = '\u{2511}';
pub const BOX_DN_H_LT_L: char = '\u{2512}';
pub const BOX_H_DN_LT: char = '\u{2513}';

pub const BOX_L_UP_RT: char = '\u{2514}';
pub const BOX_UP_L_RT_H: char = '\u{2515}';
pub const BOX_UP_H_RT_L: char = '\u{2516}';
pub const BOX_H_UP_RT: char = '\u{2517}';

pub const BOX_L_UP_LT: char = '\u{2518}';
pub const BOX_UP_L_LT_H: char = '\u{2519}';
pub const BOX_UP_H_LT_L: char = '\u{251A}';
pub const BOX_H_UP_LT: char = '\u{251B}';
