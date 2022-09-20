use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use std::ptr::null_mut;
use std::slice;
use crate::core::win32::foundation::{AsPWStr, HResult};
use crate::core::win32::graphics::direct3d12::d3d12_create_device;
use crate::core::win32::graphics::direct3d::D3DFeatureLevel;
use crate::core::win32::graphics::dxgi::{DXGI_DEBUG_ALL, DxgiAdapter, DxgiAdapterDesc1, DxgiInfoQueue, DxgiInfoQueueMessageCategory, DxgiInfoQueueMessageSeverity, RawDxgiInfoQueueMessage};
use crate::core::win32::system::com::{AsParam, GUID, IUnknown, Param, UnknownWrapper};
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

pub struct DxgiInfoQueueMessage {
    pub producer: GUID,
    pub category: DxgiInfoQueueMessageCategory,
    pub severity: DxgiInfoQueueMessageSeverity,
    pub id: i32,
    pub description: String,
}

impl DxgiInfoQueue {
    pub fn get_message(&self, producer: GUID, index: u64) -> Result<DxgiInfoQueueMessage, HResult> {
        let vt = self.as_param();
        let f: extern "system" fn(Param<Self>, producer: GUID, message_index: u64, p_message: *mut u8, p_message_byte_length: &mut usize) -> HResult
            = unsafe { transmute(vt[5]) };

        let mut length: usize = 0;
        f(vt, producer, index, null_mut(), &mut length).ok()?;

        let mut vec = Vec::<u8>::with_capacity(length);
        let vec = vec.as_mut_ptr();
        f(vt, producer, index, vec, &mut length).ok()?;

        let msg: &RawDxgiInfoQueueMessage = unsafe { transmute(vec) };
        let size = msg.description_byte_length;
        let size = if size > 0 { size - 1 } else { 0 };
        let ptr: *const u8 = unsafe { transmute(msg.description) };
        let desc = String::from_utf8_lossy(unsafe { slice::from_raw_parts(ptr, size) }).to_string();

        Ok(DxgiInfoQueueMessage {
            producer: msg.producer,
            category: msg.category,
            severity: msg.severity,
            id: msg.id,
            description: desc,
        })
    }

    pub fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(DXGI_DEBUG_ALL, DxgiInfoQueueMessageSeverity::Error, true)
    }

    pub fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(DXGI_DEBUG_ALL, DxgiInfoQueueMessageSeverity::Corruption, true)
    }
}

impl DxgiAdapter {
    pub fn create_device<T: IUnknown + From<UnknownWrapper>>(&self, minimum_feature_level: D3DFeatureLevel) -> Result<T, HResult> {
        d3d12_create_device(Some(self.as_unknown()), minimum_feature_level)
    }
}