use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;
use std::slice;
use crate::core::win32::foundation::HResult;
use crate::core::win32::graphics::direct3d::dxc::*;
use crate::core::win32::system::com::{Clsid, Com, GUID, IMalloc, IUnknown, Unknown, VTable};

pub fn DxcCreateInstance<T: IUnknown + Clsid>() -> Result<T, HResult> {
    unsafe {
        #[link(name = "dxcompiler")]
        extern "system" {
            fn DxcCreateInstance(
                rclsid: &GUID,
                riid: &GUID,
                ppv: *mut c_void,
            ) -> HResult;
        }
        let mut out: Option<Unknown> = None;
        let ret = DxcCreateInstance(
            T::CLSID,
            T::IID,
            &mut out as *mut Option<Unknown> as *mut c_void,
        );
        if ret.is_ok() {
            if let Some(o) = out {
                return Ok(T::from(o));
            }
        }
        Err(ret)
    }
}

pub fn DxcCreateInstance2<T: IUnknown + Clsid>(
    malloc: &impl IMalloc,
) -> Result<T, HResult> {
    unsafe {
        #[link(name = "dxcompiler")]
        extern "system" {
            fn DxcCreateInstance2(
                pMalloc: VTable,
                rclsid: &GUID,
                riid: &GUID,
                ppv: *mut c_void,
            ) -> HResult;
        }
        let mut out: Option<Unknown> = None;
        let ret = DxcCreateInstance2(
            malloc.vtable(),
            T::CLSID,
            T::IID,
            &mut out as *mut Option<Unknown> as *mut c_void,
        );
        if ret.is_ok() {
            if let Some(o) = out {
                return Ok(T::from(o));
            }
        }
        Err(ret)
    }
}

impl Clsid for DxcCompiler {
    const CLSID: &'static GUID = &CLSID_DxcCompiler;
}

impl Clsid for DxcCompiler2 {
    const CLSID: &'static GUID = &CLSID_DxcCompiler;
}

impl Clsid for DxcCompiler3 {
    const CLSID: &'static GUID = &CLSID_DxcCompiler;
}

impl Clsid for DxcCompilerArgs {
    const CLSID: &'static GUID = &CLSID_DxcCompilerArgs;
}

impl DxcCompilerArgs {
    pub fn new() -> Result<Self, HResult> {
        DxcCreateInstance::<DxcCompilerArgs>()
    }
}

impl DxcCompiler {
    pub fn new() -> Result<Self, HResult> {
        DxcCreateInstance::<DxcCompiler>()
    }
}

impl DxcCompiler2 {
    pub fn new() -> Result<Self, HResult> {
        DxcCreateInstance::<DxcCompiler2>()
    }
}

impl DxcCompiler3 {
    pub fn new() -> Result<Self, HResult> {
        DxcCreateInstance::<DxcCompiler3>()
    }
}

impl Deref for DxcBlob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                self.GetBufferPointer() as *const u8,
                self.GetBufferSize(),
            )
        }
    }
}

impl AsRef<[u8]> for DxcBlob {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl DxcBuffer<'_> {
    pub fn new(shader: &str) -> Self {
        DxcBuffer {
            ptr: unsafe { transmute(shader.as_ptr()) },
            size: shader.len(),
            encoding: 0xFDE9,
        }
    }
}