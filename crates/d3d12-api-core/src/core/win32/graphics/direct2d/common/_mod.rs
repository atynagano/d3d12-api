use super::*;

/// D2D1_COLOR_F
pub type D2D1ColorF = D2DColorF;

impl D2DColorF {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
}

impl D2DRectF {
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self { Self { left, top, right, bottom } }
    pub fn with_size(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self { left, top, right: left + width, bottom: top + height }
    }
}