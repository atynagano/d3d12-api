#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::foundation::*;
#[repr(C)]
pub struct DxgiOutputDuplication(pub(crate) DxgiObject);

impl Guid for DxgiOutputDuplication {
	const IID: &'static GUID = &GUID::from_u128(0x191cfac3a341470db26ea864f428319cu128);
}

impl Clone for DxgiOutputDuplication {
	fn clone(&self) -> Self { DxgiOutputDuplication(self.0.clone()) }
}

pub trait IDxgiOutputDuplication: IDxgiObject {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication;
	fn into_output_duplication(self) -> DxgiOutputDuplication;

	fn GetDesc(&self, ) -> (DxgiOutDuplDesc) {
		let vt = self.as_param();
		let mut _desc: DxgiOutDuplDesc = DxgiOutDuplDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiOutDuplDesc, ) -> ()
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, &mut _desc, );
		return (_desc);
	}

	fn AcquireNextFrame(&self, timeout_in_milliseconds: u32, ) -> Result<(DxgiOutDuplFrameInfo, DxgiResource), HResult> {
		let vt = self.as_param();
		let mut _frame_info: DxgiOutDuplFrameInfo = DxgiOutDuplFrameInfo::zeroed();
		let mut _desktop_resource: Option<DxgiResource> = None;
		let f: extern "system" fn(Param<Self>, timeout_in_milliseconds: u32, _frame_info: &mut DxgiOutDuplFrameInfo, _desktop_resource: &mut Option<DxgiResource>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, timeout_in_milliseconds, &mut _frame_info, &mut _desktop_resource, );
		if ret.is_ok() {
			if let (Some(_desktop_resource)) = (_desktop_resource) {
				return Ok((_frame_info, _desktop_resource));
			}
		}
		Err(ret)
	}

	fn GetFrameDirtyRects(&self, mut dirty_rects_buffer: &mut [Rect], ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _dirty_rects_buffer_size_required: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, dirty_rects_buffer_size: u32, dirty_rects_buffer: *mut Rect, _dirty_rects_buffer_size_required: &mut u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, size_of_val(dirty_rects_buffer) as u32, dirty_rects_buffer.as_mut_ptr_or_null(), &mut _dirty_rects_buffer_size_required, );
		if ret.is_ok() {
			return Ok((_dirty_rects_buffer_size_required));
		}
		Err(ret)
	}

	fn GetFrameMoveRects(&self, mut move_rect_buffer: &mut [DxgiOutDuplMoveRect], ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _move_rects_buffer_size_required: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, move_rects_buffer_size: u32, move_rect_buffer: *mut DxgiOutDuplMoveRect, _move_rects_buffer_size_required: &mut u32, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, size_of_val(move_rect_buffer) as u32, move_rect_buffer.as_mut_ptr_or_null(), &mut _move_rects_buffer_size_required, );
		if ret.is_ok() {
			return Ok((_move_rects_buffer_size_required));
		}
		Err(ret)
	}

	fn GetFramePointerShape(&self, mut pointer_shape_buffer: &mut [u8], ) -> Result<(u32, DxgiOutDuplPointerShapeInfo), HResult> {
		let vt = self.as_param();
		let mut _pointer_shape_buffer_size_required: u32 = u32::zeroed();
		let mut _pointer_shape_info: DxgiOutDuplPointerShapeInfo = DxgiOutDuplPointerShapeInfo::zeroed();
		let f: extern "system" fn(Param<Self>, pointer_shape_buffer_size: u32, pointer_shape_buffer: *mut u8, _pointer_shape_buffer_size_required: &mut u32, _pointer_shape_info: &mut DxgiOutDuplPointerShapeInfo, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, pointer_shape_buffer.len() as u32, pointer_shape_buffer.as_mut_ptr_or_null(), &mut _pointer_shape_buffer_size_required, &mut _pointer_shape_info, );
		if ret.is_ok() {
			return Ok((_pointer_shape_buffer_size_required, _pointer_shape_info));
		}
		Err(ret)
	}

	fn MapDesktopSurface(&self, ) -> Result<(DxgiMappedRect), HResult> {
		let vt = self.as_param();
		let mut _locked_rect: DxgiMappedRect = DxgiMappedRect::zeroed();
		let f: extern "system" fn(Param<Self>, _locked_rect: &mut DxgiMappedRect, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, &mut _locked_rect, );
		if ret.is_ok() {
			return Ok((_locked_rect));
		}
		Err(ret)
	}

	fn UnMapDesktopSurface(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn ReleaseFrame(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, );
		ret.ok()
	}
}

impl IDxgiOutputDuplication for DxgiOutputDuplication {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication { self }
	fn into_output_duplication(self) -> DxgiOutputDuplication { self }
}

impl IDxgiObject for DxgiOutputDuplication {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiOutputDuplication {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiOutputDuplication {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

