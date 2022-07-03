use crate::core::win32::graphics::dxgi::common::DxgiSampleDesc;

impl DxgiSampleDesc {
    pub const fn new(count: u32, quality: u32) -> Self {
        Self {
            count,
            quality,
        }
    }
}