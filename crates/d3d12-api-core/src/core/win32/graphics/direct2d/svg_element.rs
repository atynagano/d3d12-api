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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SvgElement(pub(crate) D2D1Resource);

impl Deref for D2D1SvgElement {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgElement {
	const IID: &'static GUID = &GUID::from_u128(0xac7b67a6183e49c1a8230ebe40b0db29u128);
}

impl Com for D2D1SvgElement {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgElement {
	pub fn GetDocument(&self, mut document: Option<&mut Option<D2D1SvgDocument>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut document) = document { **document = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(document));
		}
	}

	pub unsafe fn GetTagName(&self) { todo!() }

	pub fn GetTagNameLength(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn IsTextContent(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetParent(&self, mut parent: Option<&mut Option<D2D1SvgElement>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut parent) = parent { **parent = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(parent));
		}
	}

	pub fn HasChildren(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetFirstChild(&self, mut child: Option<&mut Option<D2D1SvgElement>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut child) = child { **child = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(child));
		}
	}

	pub fn GetLastChild(&self, mut child: Option<&mut Option<D2D1SvgElement>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut child) = child { **child = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, transmute(child));
		}
	}

	pub fn GetPreviousChild(&self, reference_child: &D2D1SvgElement, mut previous_child: Option<&mut Option<D2D1SvgElement>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut previous_child) = previous_child { **previous_child = None; }
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, reference_child.vtable(), transmute(previous_child));
			_ret_.ok()
		}
	}

	pub fn get_previous_child(&self, reference_child: &D2D1SvgElement) -> Result<D2D1SvgElement, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut previous_child_out_: Option<D2D1SvgElement> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, reference_child.vtable(), transmute(&mut previous_child_out_));
			if _ret_.is_ok() { if let Some(previous_child_out_) = previous_child_out_ { return Ok(previous_child_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetNextChild(&self, reference_child: &D2D1SvgElement, mut next_child: Option<&mut Option<D2D1SvgElement>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut next_child) = next_child { **next_child = None; }
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, reference_child.vtable(), transmute(next_child));
			_ret_.ok()
		}
	}

	pub fn get_next_child(&self, reference_child: &D2D1SvgElement) -> Result<D2D1SvgElement, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut next_child_out_: Option<D2D1SvgElement> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, reference_child.vtable(), transmute(&mut next_child_out_));
			if _ret_.is_ok() { if let Some(next_child_out_) = next_child_out_ { return Ok(next_child_out_); } }
			Err(_ret_)
		}
	}

	pub fn InsertChildBefore(&self, new_child: &D2D1SvgElement, reference_child: Option<&D2D1SvgElement>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, new_child.vtable(), option_to_vtable(reference_child));
			_ret_.ok()
		}
	}

	pub fn AppendChild(&self, new_child: &D2D1SvgElement) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, new_child.vtable());
			_ret_.ok()
		}
	}

	pub fn ReplaceChild(&self, new_child: &D2D1SvgElement, old_child: &D2D1SvgElement) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, new_child.vtable(), old_child.vtable());
			_ret_.ok()
		}
	}

	pub fn RemoveChild(&self, old_child: &D2D1SvgElement) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, old_child.vtable());
			_ret_.ok()
		}
	}

	pub fn CreateChild(&self, tag_name: &str) -> Result<D2D1SvgElement, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut new_child_out_: Option<D2D1SvgElement> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, tag_name.to_utf16().as_ptr_or_null(), transmute(&mut new_child_out_));
			if _ret_.is_ok() { if let Some(new_child_out_) = new_child_out_ { return Ok(new_child_out_); } }
			Err(_ret_)
		}
	}

	pub fn IsAttributeSpecified(&self, name: &str, inherited: Option<&mut MaybeUninit<Bool>>) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, Option<&mut MaybeUninit<Bool>>) -> Bool
				= transmute(vt[19]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), inherited);
			_ret_.to_bool()
		}
	}

	pub fn is_attribute_specified(&self, name: &str) -> (bool, bool) {
		unsafe {
			let vt = self.as_param();
			let mut inherited_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, *const u16, &mut Bool) -> Bool
				= transmute(vt[19]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), &mut inherited_out_);
			(_ret_.to_bool(), inherited_out_.to_bool())
		}
	}

	pub fn GetSpecifiedAttributeCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[20]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetSpecifiedAttributeName(&self) { todo!() }

	pub fn GetSpecifiedAttributeNameLength(&self, index: u32, inherited: Option<&mut MaybeUninit<Bool>>) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, Option<&mut MaybeUninit<Bool>>) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, index, name_length_out_.as_mut_ptr(), inherited);
			if _ret_.is_ok() { Ok(name_length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn get_specified_attribute_name_length(&self, index: u32) -> Result<(u32, bool), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut name_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut inherited_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, u32, *mut u32, &mut Bool) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, index, name_length_out_.as_mut_ptr(), &mut inherited_out_);
			if _ret_.is_ok() { Ok((name_length_out_.assume_init(), inherited_out_.to_bool())) } else { Err(_ret_) }
		}
	}

	pub fn RemoveAttribute(&self, name: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}

	pub fn SetTextValue(&self, name: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let name_utf16_ = name.encode_utf16().collect::<Vec<_>>();
			let (name_ptr_, name_len_) = name_utf16_.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u16, u32) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, name_ptr_, name_len_ as u32);
			_ret_.ok()
		}
	}

	pub unsafe fn GetTextValue(&self) { todo!() }

	pub fn GetTextValueLength(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[26]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetAttributeValue(&self, name: &str, value: &D2D1SvgAttribute) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, VTable) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), value.vtable());
			_ret_.ok()
		}
	}

	pub fn GetAttributeValue<T: IUnknown + From<UnknownWrapper>>(&self, name: &str, value: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut value_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, *const u16, &GUID, *mut c_void) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), T::IID, transmute(if value_out_.is_some() { Some(&mut value_out_) } else { None }));
			if let Some(value_out_) = value_out_ { *value.unwrap_unchecked() = Some(T::from(UnknownWrapper(value_out_))); }
			_ret_.ok()
		}
	}

	pub fn get_attribute_value<T: IUnknown + From<UnknownWrapper>>(&self, name: &str) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut value_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, *const u16, &GUID, *mut c_void) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), T::IID, transmute(&mut value_out_));
			if _ret_.is_ok() { if let Some(value_out_) = value_out_ { return Ok(T::from(UnknownWrapper(value_out_))); } }
			Err(_ret_)
		}
	}

	pub fn GetAttributeValueLength(&self, name: &str, r#type: D2D1SvgAttributeStringType) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut value_length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const u16, D2D1SvgAttributeStringType, *mut u32) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), r#type, value_length_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(value_length_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1SvgElement: ID2D1Resource {
	fn as_svg_element(&self) -> &D2D1SvgElement;
	fn into_svg_element(self) -> D2D1SvgElement;
}

impl ID2D1SvgElement for D2D1SvgElement {
	fn as_svg_element(&self) -> &D2D1SvgElement { self }
	fn into_svg_element(self) -> D2D1SvgElement { self }
}
impl ID2D1Resource for D2D1SvgElement {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgElement {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgElement {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

