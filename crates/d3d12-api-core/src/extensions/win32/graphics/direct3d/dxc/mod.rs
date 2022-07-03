use std::ffi::c_void;
use std::mem::transmute;
use std::slice;
use crate::aliases::win32::graphics::direct3d::dxc::BlobEncoding;
use crate::core::win32::foundation::{HResult, OkOrErr, PWStr};
use crate::core::win32::graphics::direct3d::dxc::{DxcBlob, DxcBuffer, IDxcBlob, IDxcCompilerArgs, IDxcIncludeHandler, IDxcOperationResult};
use crate::core::win32::system::com::{AsParam, GUID, IUnknown, option_to_vtable, Param, Unknown, VTable};
use crate::helpers::{Len, ToUtf16Vec};

pub trait IDxcOperationResultEx: IDxcOperationResult {
    fn get_result(&self) -> Result<DxcBlob, HResult>;
    fn get_error_buffer(&self) -> Result<BlobEncoding, HResult>;
}

impl<T: IDxcOperationResult> IDxcOperationResultEx for T {
    fn get_result(&self) -> Result<DxcBlob, HResult> {
        let mut blob: Option<DxcBlob> = None;
        self.GetResult(Some(&mut blob))?;
        blob.ok_or_err()
    }

    fn get_error_buffer(&self) -> Result<BlobEncoding, HResult> {
        let mut blob: Option<BlobEncoding> = None;
        self.GetErrorBuffer(Some(&mut blob))?;
        blob.ok_or_err()
    }
}

pub trait IDxcBlobEx {
    // fn as_str(&self) -> &str;
    fn to_string(&self) -> String;
}

impl<T: IDxcBlob> IDxcBlobEx for T {
    fn to_string(&self) -> String {
        unsafe {
            let len = self.GetBufferSize();
            let ptr = self.GetBufferPointer() as *const c_void as *const u8;
            let slice = slice::from_raw_parts(
                ptr,
                if len > 0 { len - 1 } else { 0 },
            );
            String::from_utf8_lossy(slice).to_string()
        }
    }
}


pub trait IDxcCompilerArgsEx {
    fn get_arguments(&self) -> &[PWStr];
}


impl<T: IDxcCompilerArgs> IDxcCompilerArgsEx for T {
    fn get_arguments(&self) -> &[PWStr] {
        unsafe {
            slice::from_raw_parts(
                self.GetArguments() as *const PWStr,
                self.GetCount() as usize,
            )
        }
    }
}

// note: ?
/*
pub trait IDxcCompiler3Ex {
    fn compile<T: IUnknown>(&self, source: &DxcBuffer, arguments: Option<&[&str]>, include_handler: Option<&impl IDxcIncludeHandler>) -> Result<(T), HResult>;
}

impl<T: IDxcCompilerArgs3> IDxcCompilerArgs3Ex for T {
    fn compile<U: IUnknown>(&self, source: &DxcBuffer, arguments: Option<&[&str]>, include_handler: Option<&impl IDxcIncludeHandler>) -> Result<(T), HResult> {
        let vt = self.as_param();
        let mut _result: Option<Unknown> = None;
        let f: extern "system" fn(Param<Self>, source: &DxcBuffer, arguments: *const PWStr, arg_count: u32, include_handler: Option<VTable>, riid: &GUID, _result: &mut Option<Unknown>) -> HResult
            = unsafe { transmute(vt[3]) };
        let ret = f(vt, source, arguments.to_utf16_vec().ptr(), arguments.len() as u32, option_to_vtable(include_handler), T::IID, &mut _result);
        if ret.is_ok() {
            if let (Some(_result)) = (_result) {
                return Ok((T::from(_result)));
            }
        }
        Err(ret)
    }
}*/