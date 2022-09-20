use std::ffi::c_void;
use std::mem::transmute;
use crate::core::win32::foundation::HResult;
use crate::core::win32::system::com::{GUID, IUnknown, Unknown, UnknownWrapper};
use super::*;

pub fn D2D1CreateFactory<T: IUnknown + From<UnknownWrapper>>(r#type: D2D1FactoryType, options: Option<&D2D1FactoryOptions>) -> Result<T, HResult> {
    unsafe {
        #[link(name = "d2d1")]
        extern "system" {
            fn D2D1CreateFactory(ty: D2D1FactoryType, riid: &GUID, options: Option<&D2D1FactoryOptions>, factory_out_: *mut c_void) -> HResult;
        }
        let mut factory_out_: Option<Unknown> = None;
        let _ret_ = D2D1CreateFactory(r#type, T::IID, options, transmute(&mut factory_out_));
        if _ret_.is_ok() {
            if let Some(factory_out_) = factory_out_ {
                return Ok(T::from(UnknownWrapper(factory_out_)));
            }
        }
        Err(_ret_)
    }
}