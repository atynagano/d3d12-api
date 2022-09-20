use std::mem::transmute;
use std::ops::Deref;
use std::ptr::null;
use std::slice;
use crate::core::win32::graphics::direct3d::{D3DBlob, ID3DBlob};

impl AsRef<[u8]> for D3DBlob {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl D3DBlob {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = self.GetBufferPointer() as _;
            let len = self.GetBufferSize();
            if ptr == null() || len == 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr, len)
            }
        }
    }
}