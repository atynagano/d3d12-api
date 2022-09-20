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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1TransformGraph(pub(crate) Unknown);

impl Deref for D2D1TransformGraph {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1TransformGraph {
	const IID: &'static GUID = &GUID::from_u128(0x13d29038c3e64034908113b53a417992u128);
}

impl Com for D2D1TransformGraph {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1TransformGraph {
	pub fn GetInputCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetSingleTransformNode(&self, node: &D2D1TransformNode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, node.vtable());
			_ret_.ok()
		}
	}

	pub fn AddNode(&self, node: &D2D1TransformNode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, node.vtable());
			_ret_.ok()
		}
	}

	pub fn RemoveNode(&self, node: &D2D1TransformNode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, node.vtable());
			_ret_.ok()
		}
	}

	pub fn SetOutputNode(&self, node: &D2D1TransformNode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, node.vtable());
			_ret_.ok()
		}
	}

	pub fn ConnectNode(&self, from_node: &D2D1TransformNode, to_node: &D2D1TransformNode, to_node_input_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, from_node.vtable(), to_node.vtable(), to_node_input_index);
			_ret_.ok()
		}
	}

	pub fn ConnectToEffectInput(&self, to_effect_input_index: u32, node: &D2D1TransformNode, to_node_input_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, VTable, u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, to_effect_input_index, node.vtable(), to_node_input_index);
			_ret_.ok()
		}
	}

	pub fn Clear(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt);
		}
	}

	pub fn SetPassthroughGraph(&self, effect_input_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, effect_input_index);
			_ret_.ok()
		}
	}
}

pub trait ID2D1TransformGraph: IUnknown {
	fn as_transform_graph(&self) -> &D2D1TransformGraph;
	fn into_transform_graph(self) -> D2D1TransformGraph;
}

impl ID2D1TransformGraph for D2D1TransformGraph {
	fn as_transform_graph(&self) -> &D2D1TransformGraph { self }
	fn into_transform_graph(self) -> D2D1TransformGraph { self }
}
impl IUnknown for D2D1TransformGraph {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1TransformGraph {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

