use std::fmt::{Debug, Formatter};
use crate::core::win32::foundation::AsPWStr;
use crate::core::win32::graphics::dxgi::DxgiAdapterDesc1;

impl Debug for DxgiAdapterDesc1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
DxgiAdapterDesc1 {{
    description: {:?},
    vendor_id: {:?},
    device_id: {:?},
    sub_sys_id: {:?},
    revision: {:?},
    dedicated_video_memory: {:?},
    dedicated_system_memory: {:?},
    shared_system_memory: {:?},
    adapter_luid: Luid {{ low_part: {:?}, high_part: {:?} }},
    flags: {:?}
}}",
               self.description.as_pwstr(),
               self.vendor_id,
               self.device_id,
               self.sub_sys_id,
               self.revision,
               self.dedicated_video_memory,
               self.dedicated_system_memory,
               self.shared_system_memory,
               self.adapter_luid.low_part,
               self.adapter_luid.high_part,
               self.flags)
    }
}