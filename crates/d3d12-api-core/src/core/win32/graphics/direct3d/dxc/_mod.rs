use std::borrow::Cow;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use std::ops::Deref;
use std::slice;
use crate::core::win32::foundation::HResult;
use crate::core::win32::graphics::direct3d::dxc::*;
use crate::core::win32::system::com::{Clsid, Com, GUID, IMalloc, IUnknown, Unknown, UnknownWrapper, VTable};

pub fn DxcCreateInstance<T: IUnknown + From<UnknownWrapper> + Clsid>() -> Result<T, HResult> {
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
                return Ok(T::from(UnknownWrapper(o)));
            }
        }
        Err(ret)
    }
}

pub fn DxcCreateInstance2<T: IUnknown + From<UnknownWrapper> + Clsid>(
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
                return Ok(T::from(UnknownWrapper(o)));
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

impl AsRef<[u8]> for DxcBlob {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DxcBlob {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = self.GetBufferPointer() as _;
            let len = self.GetBufferSize();
            if ptr == std::ptr::null() || len == 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr, len)
            }
        }
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

impl Debug for DxcBlob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Cow<str> as Debug>::fmt(&String::from_utf8_lossy(self.as_slice()), f)
    }
}

impl Display for DxcBlob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Cow<str> as Display>::fmt(&String::from_utf8_lossy(self.as_slice()), f)
    }
}

impl Debug for DxcBlobEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <DxcBlob as Debug>::fmt(self.as_blob(), f)
    }
}

impl Display for DxcBlobEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <DxcBlob as Display>::fmt(self.as_blob(), f)
    }
}