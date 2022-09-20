use super::*;

impl Margins {
    // note: css order
    pub const fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            cx_left_width: left,
            cx_right_width: right,
            cy_top_height: top,
            cy_bottom_height: bottom,
        }
    }
}