use std::mem::transmute;
use std::ptr::null_mut;
use std::slice;
use crate::core::win32::foundation::HResult;
use crate::aliases::win32::graphics::dxgi::{IInfoQueue, InfoQueueMessage, InfoQueueMessageCategory, InfoQueueMessageSeverity};
use crate::core::win32::graphics::dxgi::{DXGI_DEBUG_ALL, DxgiInfoQueueMessageSeverity};
use crate::core::win32::system::com::{AsParam, GUID, Param};

pub type Message = DxgiMessage;
pub struct DxgiMessage {
    pub producer: GUID,
    pub category: InfoQueueMessageCategory,
    pub severity: InfoQueueMessageSeverity,
    pub id: i32,
    pub description: String,
}

pub trait IDxgiInfoQueueEx {
    fn get_message(&self, producer: GUID, index: u64) -> Result<DxgiMessage, HResult>;
    fn enable_break_on_error(&self) -> Result<(), HResult>;
    fn enable_break_on_corruption(&self) -> Result<(), HResult>;
}

impl<T: IInfoQueue> IDxgiInfoQueueEx for T {
    fn get_message(&self, producer: GUID, index: u64) -> Result<DxgiMessage, HResult> {
        let vt = self.as_param();
        let f: extern "system" fn(Param<Self>, producer: GUID, message_index: u64, p_message: *mut u8, p_message_byte_length: &mut usize) -> HResult
            = unsafe { transmute(vt[5]) };
        
        let mut length: usize = 0;
        f(vt, producer, index, null_mut(), &mut length).ok()?;
        
        let mut vec = Vec::<u8>::with_capacity(length);
        let vec = vec.as_mut_ptr();
        f(vt, producer, index, vec, &mut length).ok()?;
        
        let msg: &InfoQueueMessage = unsafe { transmute(vec) };
        let size = msg.description_byte_length;
        let size = if size > 0 { size - 1 } else { 0 };
        let ptr: *const u8 = unsafe { transmute(msg.description) };
        let desc = String::from_utf8_lossy(unsafe { slice::from_raw_parts(ptr, size) }).to_string();
        
        Ok(DxgiMessage {
            producer: msg.producer,
            category: msg.category,
            severity: msg.severity,
            id: msg.id,
            description: desc,
        })
    }

    fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(DXGI_DEBUG_ALL, DxgiInfoQueueMessageSeverity::Error, true)
    }

    fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(DXGI_DEBUG_ALL, DxgiInfoQueueMessageSeverity::Corruption, true)
    }
}

