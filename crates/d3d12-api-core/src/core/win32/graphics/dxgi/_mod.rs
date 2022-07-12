use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use crate::core::win32::foundation::AsPWStr;
use crate::core::win32::graphics::dxgi::DxgiAdapterDesc1;
use crate::helpers::{DebugWithSuffix, DebugStringWithoutQuot, ToUtf16};

impl DxgiAdapterDesc1 {
    pub fn description(&self) -> String {
        self.description.to_utf16().to_string()
    }
}

impl Debug for DxgiAdapterDesc1 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        //
        let description = self.description();
        let vendor: &str = match self.vendor_id {
            0x10DE => "NVIDIA",
            0x1002 | 0x1022 => "AMD",
            0x163C | 0x8086 | 0x8087 => "Intel",
            _ => "Unknown",
        };
        let vendor = DebugStringWithoutQuot::new(vendor);

        let dedicated_video_memory = self.dedicated_video_memory / (1024 * 1024);
        let dedicated_system_memory = self.dedicated_system_memory / (1024 * 1024);
        let shared_system_memory = self.shared_system_memory / (1024 * 1024);

        let dedicated_video_memory = DebugWithSuffix::new(&dedicated_video_memory, " MiB");
        let dedicated_system_memory = DebugWithSuffix::new(&dedicated_system_memory, " MiB");
        let shared_system_memory = DebugWithSuffix::new(&shared_system_memory, " MiB");

        f.debug_struct("DxgiAdapterDesc1")
            .field("description", &description)
            .field("vendor", &vendor)
            .field("device_id", &self.device_id)
            .field("sub_sys_id", &self.sub_sys_id)
            .field("revision", &self.revision)
            .field("dedicated_video_memory", &dedicated_video_memory)
            .field("dedicated_system_memory", &dedicated_system_memory)
            .field("shared_system_memory", &shared_system_memory)
            .field("adapter_luid", &self.adapter_luid)
            .field("flags", &self.flags)
            .finish()
    }
}