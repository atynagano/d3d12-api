#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dwm::*;
use crate::core::win32::ui::controls::*;
use crate::core::win32::graphics::gdi::*;


pub fn DwmDefWindowProc(wnd: HWnd, msg: u32, w_param: WParam, l_param: LParam) -> (bool, LResult) {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmDefWindowProc(hWnd: HWnd, msg: u32, wParam: WParam, lParam: LParam, plResult: *mut LResult) -> Bool;
		} 
		let mut pl_result_out_: MaybeUninit<LResult> = MaybeUninit::zeroed();
		let _ret_ = DwmDefWindowProc(wnd, msg, w_param, l_param, pl_result_out_.as_mut_ptr());
		(_ret_.to_bool(), pl_result_out_.assume_init())
	}
}

pub fn DwmEnableBlurBehindWindow(wnd: HWnd, blur_behind: &DwmBlurBehind) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmEnableBlurBehindWindow(hWnd: HWnd, pBlurBehind: &DwmBlurBehind) -> HResult;
		} 
		let _ret_ = DwmEnableBlurBehindWindow(wnd, blur_behind);
		_ret_.ok()
	}
}

pub fn DwmEnableComposition(u_composition_action: u32) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmEnableComposition(uCompositionAction: u32) -> HResult;
		} 
		let _ret_ = DwmEnableComposition(u_composition_action);
		_ret_.ok()
	}
}

pub fn DwmEnableMMCSS(f_enable_mmcss: bool) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmEnableMMCSS(fEnableMMCSS: Bool) -> HResult;
		} 
		let _ret_ = DwmEnableMMCSS(f_enable_mmcss.to_bool());
		_ret_.ok()
	}
}

pub fn DwmExtendFrameIntoClientArea(wnd: HWnd, mar_inset: &Margins) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmExtendFrameIntoClientArea(hWnd: HWnd, pMarInset: &Margins) -> HResult;
		} 
		let _ret_ = DwmExtendFrameIntoClientArea(wnd, mar_inset);
		_ret_.ok()
	}
}

pub fn DwmGetColorizationColor() -> Result<(u32, bool), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmGetColorizationColor(pcrColorization: *mut u32, pfOpaqueBlend: &mut Bool) -> HResult;
		} 
		let mut pcr_colorization_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let mut pf_opaque_blend_out_ = Bool::FALSE;
		let _ret_ = DwmGetColorizationColor(pcr_colorization_out_.as_mut_ptr(), &mut pf_opaque_blend_out_);
		if _ret_.is_ok() { Ok((pcr_colorization_out_.assume_init(), pf_opaque_blend_out_.to_bool())) } else { Err(_ret_) }
	}
}

pub fn DwmGetCompositionTimingInfo(hwnd: HWnd) -> Result<DwmTimingInfo, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmGetCompositionTimingInfo(hwnd: HWnd, pTimingInfo: *mut DwmTimingInfo) -> HResult;
		} 
		let mut timing_info_out_: MaybeUninit<DwmTimingInfo> = MaybeUninit::zeroed();
		let _ret_ = DwmGetCompositionTimingInfo(hwnd, timing_info_out_.as_mut_ptr());
		if _ret_.is_ok() { Ok(timing_info_out_.assume_init()) } else { Err(_ret_) }
	}
}

pub unsafe fn DwmGetWindowAttribute() { todo!() }

pub fn DwmIsCompositionEnabled() -> Result<bool, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmIsCompositionEnabled(pfEnabled: &mut Bool) -> HResult;
		} 
		let mut pf_enabled_out_ = Bool::FALSE;
		let _ret_ = DwmIsCompositionEnabled(&mut pf_enabled_out_);
		if _ret_.is_ok() { Ok(pf_enabled_out_.to_bool()) } else { Err(_ret_) }
	}
}

pub fn DwmModifyPreviousDxFrameDuration(hwnd: HWnd, refreshes: i32, f_relative: bool) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmModifyPreviousDxFrameDuration(hwnd: HWnd, cRefreshes: i32, fRelative: Bool) -> HResult;
		} 
		let _ret_ = DwmModifyPreviousDxFrameDuration(hwnd, refreshes, f_relative.to_bool());
		_ret_.ok()
	}
}

pub fn DwmQueryThumbnailSourceSize(thumbnail: NonZeroUsize) -> Result<Size, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmQueryThumbnailSourceSize(hThumbnail: NonZeroUsize, pSize: *mut Size) -> HResult;
		} 
		let mut size_out_: MaybeUninit<Size> = MaybeUninit::zeroed();
		let _ret_ = DwmQueryThumbnailSourceSize(thumbnail, size_out_.as_mut_ptr());
		if _ret_.is_ok() { Ok(size_out_.assume_init()) } else { Err(_ret_) }
	}
}

pub fn DwmRegisterThumbnail(hwnd_destination: HWnd, hwnd_source: HWnd) -> Result<NonZeroUsize, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmRegisterThumbnail(hwndDestination: HWnd, hwndSource: HWnd, phThumbnailId: *mut c_void) -> HResult;
		} 
		let mut ph_thumbnail_id_out_: Option<NonZeroUsize> = None;
		let _ret_ = DwmRegisterThumbnail(hwnd_destination, hwnd_source, transmute(&mut ph_thumbnail_id_out_));
		if _ret_.is_ok() { if let Some(ph_thumbnail_id_out_) = ph_thumbnail_id_out_ { return Ok(ph_thumbnail_id_out_); } }
		Err(_ret_)
	}
}

pub fn DwmSetDxFrameDuration(hwnd: HWnd, refreshes: i32) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmSetDxFrameDuration(hwnd: HWnd, cRefreshes: i32) -> HResult;
		} 
		let _ret_ = DwmSetDxFrameDuration(hwnd, refreshes);
		_ret_.ok()
	}
}

pub fn DwmSetPresentParameters(hwnd: HWnd, present_params: &mut DwmPresentParameters) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmSetPresentParameters(hwnd: HWnd, pPresentParams: &mut DwmPresentParameters) -> HResult;
		} 
		let _ret_ = DwmSetPresentParameters(hwnd, present_params);
		_ret_.ok()
	}
}

pub fn DwmUnregisterThumbnail(thumbnail_id: NonZeroUsize) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmUnregisterThumbnail(hThumbnailId: NonZeroUsize) -> HResult;
		} 
		let _ret_ = DwmUnregisterThumbnail(thumbnail_id);
		_ret_.ok()
	}
}

pub fn DwmUpdateThumbnailProperties(thumbnail_id: NonZeroUsize, ptn_properties: &DwmThumbnailProperties) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmUpdateThumbnailProperties(hThumbnailId: NonZeroUsize, ptnProperties: &DwmThumbnailProperties) -> HResult;
		} 
		let _ret_ = DwmUpdateThumbnailProperties(thumbnail_id, ptn_properties);
		_ret_.ok()
	}
}

pub fn DwmSetIconicThumbnail(hwnd: HWnd, hbmp: HBitmap, sit_flags: u32) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmSetIconicThumbnail(hwnd: HWnd, hbmp: HBitmap, dwSITFlags: u32) -> HResult;
		} 
		let _ret_ = DwmSetIconicThumbnail(hwnd, hbmp, sit_flags);
		_ret_.ok()
	}
}

pub fn DwmSetIconicLivePreviewBitmap(hwnd: HWnd, hbmp: HBitmap, ppt_client: Option<&Point>, sit_flags: u32) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmSetIconicLivePreviewBitmap(hwnd: HWnd, hbmp: HBitmap, pptClient: *const c_void, dwSITFlags: u32) -> HResult;
		} 
		let _ret_ = DwmSetIconicLivePreviewBitmap(hwnd, hbmp, transmute(ppt_client), sit_flags);
		_ret_.ok()
	}
}

pub fn DwmInvalidateIconicBitmaps(hwnd: HWnd) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmInvalidateIconicBitmaps(hwnd: HWnd) -> HResult;
		} 
		let _ret_ = DwmInvalidateIconicBitmaps(hwnd);
		_ret_.ok()
	}
}

pub fn DwmAttachMilContent(hwnd: HWnd) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmAttachMilContent(hwnd: HWnd) -> HResult;
		} 
		let _ret_ = DwmAttachMilContent(hwnd);
		_ret_.ok()
	}
}

pub fn DwmDetachMilContent(hwnd: HWnd) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmDetachMilContent(hwnd: HWnd) -> HResult;
		} 
		let _ret_ = DwmDetachMilContent(hwnd);
		_ret_.ok()
	}
}

pub fn DwmFlush() -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmFlush() -> HResult;
		} 
		let _ret_ = DwmFlush();
		_ret_.ok()
	}
}

pub fn DwmGetGraphicsStreamTransformHint(u_index: u32) -> Result<MilMatrix3x2D, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmGetGraphicsStreamTransformHint(uIndex: u32, pTransform: *mut MilMatrix3x2D) -> HResult;
		} 
		let mut transform_out_: MaybeUninit<MilMatrix3x2D> = MaybeUninit::zeroed();
		let _ret_ = DwmGetGraphicsStreamTransformHint(u_index, transform_out_.as_mut_ptr());
		if _ret_.is_ok() { Ok(transform_out_.assume_init()) } else { Err(_ret_) }
	}
}

pub unsafe fn DwmGetGraphicsStreamClient() { todo!() }

pub fn DwmGetTransportAttributes() -> Result<(bool, bool, u32), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmGetTransportAttributes(pfIsRemoting: &mut Bool, pfIsConnected: &mut Bool, pDwGeneration: *mut u32) -> HResult;
		} 
		let mut pf_is_remoting_out_ = Bool::FALSE;
		let mut pf_is_connected_out_ = Bool::FALSE;
		let mut dw_generation_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
		let _ret_ = DwmGetTransportAttributes(&mut pf_is_remoting_out_, &mut pf_is_connected_out_, dw_generation_out_.as_mut_ptr());
		if _ret_.is_ok() { Ok((pf_is_remoting_out_.to_bool(), pf_is_connected_out_.to_bool(), dw_generation_out_.assume_init())) } else { Err(_ret_) }
	}
}

pub fn DwmTransitionOwnedWindow(hwnd: HWnd, target: DwmTransitionOwnedWindowTarget) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmTransitionOwnedWindow(hwnd: HWnd, target: DwmTransitionOwnedWindowTarget) -> HResult;
		} 
		let _ret_ = DwmTransitionOwnedWindow(hwnd, target);
		_ret_.ok()
	}
}

	pub unsafe fn DwmRenderGesture() { todo!() }

pub fn DwmTetherContact(pointer_id: u32, f_enable: bool, pt_tether: Point) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmTetherContact(dwPointerID: u32, fEnable: Bool, ptTether: Point) -> HResult;
		} 
		let _ret_ = DwmTetherContact(pointer_id, f_enable.to_bool(), pt_tether);
		_ret_.ok()
	}
}

pub fn DwmShowContact(pointer_id: u32, e_show_contact: DwmShowContact) -> Result<(), HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmShowContact(dwPointerID: u32, eShowContact: DwmShowContact) -> HResult;
		} 
		let _ret_ = DwmShowContact(pointer_id, e_show_contact);
		_ret_.ok()
	}
}

pub fn DwmGetUnmetTabRequirements(app_window: Option<HWnd>) -> Result<DwmTabWindowRequirements, HResult> {
	unsafe {
		#[link(name = "dwmapi")]
		extern "system" {
			fn DwmGetUnmetTabRequirements(appWindow: *const c_void, value: *mut DwmTabWindowRequirements) -> HResult;
		} 
		let mut value_out_: MaybeUninit<DwmTabWindowRequirements> = MaybeUninit::zeroed();
		let _ret_ = DwmGetUnmetTabRequirements(transmute(app_window), value_out_.as_mut_ptr());
		if _ret_.is_ok() { Ok(value_out_.assume_init()) } else { Err(_ret_) }
	}
}


