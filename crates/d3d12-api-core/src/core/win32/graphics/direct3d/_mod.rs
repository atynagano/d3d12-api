use std::mem::transmute;
use std::ops::Deref;
use std::ptr::null;
use std::slice;
use crate::core::win32::graphics::direct3d::{D3DBlob, ID3DBlob};

impl Deref for D3DBlob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr: *const u8 = transmute(self.GetBufferPointer());
            let len = self.GetBufferSize();
            if ptr == null() || len == 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr, len)
            }
        }
    }
}

impl AsRef<[u8]> for D3DBlob {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl D3DBlob {
    pub fn as_slice(&self) -> &[u8] {
        self.deref()
    }
}