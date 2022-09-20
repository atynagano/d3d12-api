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
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::system::com::*;


pub fn D2D1MakeRotateMatrix(angle: f32, center: D2DPoint2F) -> D2DMatrix3x2F {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1MakeRotateMatrix(angle: f32, center: D2DPoint2F, matrix: *mut D2DMatrix3x2F) -> ();
		} 
		let mut matrix_out_: MaybeUninit<D2DMatrix3x2F> = MaybeUninit::zeroed();
		let _ret_ = D2D1MakeRotateMatrix(angle, center, matrix_out_.as_mut_ptr());
		matrix_out_.assume_init()
	}
}

pub fn D2D1MakeSkewMatrix(angle_x: f32, angle_y: f32, center: D2DPoint2F) -> D2DMatrix3x2F {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1MakeSkewMatrix(angleX: f32, angleY: f32, center: D2DPoint2F, matrix: *mut D2DMatrix3x2F) -> ();
		} 
		let mut matrix_out_: MaybeUninit<D2DMatrix3x2F> = MaybeUninit::zeroed();
		let _ret_ = D2D1MakeSkewMatrix(angle_x, angle_y, center, matrix_out_.as_mut_ptr());
		matrix_out_.assume_init()
	}
}

pub fn D2D1IsMatrixInvertible(matrix: &D2DMatrix3x2F) -> bool {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1IsMatrixInvertible(matrix: &D2DMatrix3x2F) -> Bool;
		} 
		let _ret_ = D2D1IsMatrixInvertible(matrix);
		_ret_.to_bool()
	}
}

pub fn D2D1InvertMatrix(matrix: &mut D2DMatrix3x2F) -> bool {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1InvertMatrix(matrix: &mut D2DMatrix3x2F) -> Bool;
		} 
		let _ret_ = D2D1InvertMatrix(matrix);
		_ret_.to_bool()
	}
}

pub fn D2D1CreateDevice(dxgi_device: &DxgiDevice, creation_properties: Option<&D2D1CreationProperties>) -> Result<D2D1Device, HResult> {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1CreateDevice(dxgiDevice: VTable, creationProperties: *const c_void, d2dDevice: *mut c_void) -> HResult;
		} 
		let mut d2d_device_out_: Option<D2D1Device> = None;
		let _ret_ = D2D1CreateDevice(dxgi_device.vtable(), transmute(creation_properties), transmute(&mut d2d_device_out_));
		if _ret_.is_ok() { if let Some(d2d_device_out_) = d2d_device_out_ { return Ok(d2d_device_out_); } }
		Err(_ret_)
	}
}

pub fn D2D1CreateDeviceContext(dxgi_surface: &DxgiSurface, creation_properties: Option<&D2D1CreationProperties>) -> Result<D2D1DeviceContext, HResult> {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1CreateDeviceContext(dxgiSurface: VTable, creationProperties: *const c_void, d2dDeviceContext: *mut c_void) -> HResult;
		} 
		let mut d2d_device_context_out_: Option<D2D1DeviceContext> = None;
		let _ret_ = D2D1CreateDeviceContext(dxgi_surface.vtable(), transmute(creation_properties), transmute(&mut d2d_device_context_out_));
		if _ret_.is_ok() { if let Some(d2d_device_context_out_) = d2d_device_context_out_ { return Ok(d2d_device_context_out_); } }
		Err(_ret_)
	}
}

pub fn D2D1ConvertColorSpace(source_color_space: D2D1ColorSpace, destination_color_space: D2D1ColorSpace, color: &D2D1ColorF) -> D2D1ColorF {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1ConvertColorSpace(sourceColorSpace: D2D1ColorSpace, destinationColorSpace: D2D1ColorSpace, color: &D2D1ColorF) -> D2D1ColorF;
		} 
		let _ret_ = D2D1ConvertColorSpace(source_color_space, destination_color_space, color);
		_ret_
	}
}

pub fn D2D1SinCos(angle: f32) -> (f32, f32) {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32) -> ();
		} 
		let mut s_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let mut c_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
		let _ret_ = D2D1SinCos(angle, s_out_.as_mut_ptr(), c_out_.as_mut_ptr());
		(s_out_.assume_init(), c_out_.assume_init())
	}
}

pub fn D2D1Tan(angle: f32) -> f32 {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1Tan(angle: f32) -> f32;
		} 
		let _ret_ = D2D1Tan(angle);
		_ret_
	}
}

pub fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32 {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32;
		} 
		let _ret_ = D2D1Vec3Length(x, y, z);
		_ret_
	}
}

pub fn D2D1ComputeMaximumScaleFactor(matrix: &D2DMatrix3x2F) -> f32 {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1ComputeMaximumScaleFactor(matrix: &D2DMatrix3x2F) -> f32;
		} 
		let _ret_ = D2D1ComputeMaximumScaleFactor(matrix);
		_ret_
	}
}

pub fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(point0: &D2DPoint2F, point1: &D2DPoint2F, point2: &D2DPoint2F, point3: &D2DPoint2F, point4: &D2DPoint2F, point5: &D2DPoint2F, point6: &D2DPoint2F, point7: &D2DPoint2F, point8: &D2DPoint2F, point9: &D2DPoint2F, point10: &D2DPoint2F, point11: &D2DPoint2F) -> (D2DPoint2F, D2DPoint2F, D2DPoint2F, D2DPoint2F) {
	unsafe {
		#[link(name = "d2d1")]
		extern "system" {
			fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(pPoint0: &D2DPoint2F, pPoint1: &D2DPoint2F, pPoint2: &D2DPoint2F, pPoint3: &D2DPoint2F, pPoint4: &D2DPoint2F, pPoint5: &D2DPoint2F, pPoint6: &D2DPoint2F, pPoint7: &D2DPoint2F, pPoint8: &D2DPoint2F, pPoint9: &D2DPoint2F, pPoint10: &D2DPoint2F, pPoint11: &D2DPoint2F, pTensorPoint11: *mut D2DPoint2F, pTensorPoint12: *mut D2DPoint2F, pTensorPoint21: *mut D2DPoint2F, pTensorPoint22: *mut D2DPoint2F) -> ();
		} 
		let mut tensor_point11_out_: MaybeUninit<D2DPoint2F> = MaybeUninit::zeroed();
		let mut tensor_point12_out_: MaybeUninit<D2DPoint2F> = MaybeUninit::zeroed();
		let mut tensor_point21_out_: MaybeUninit<D2DPoint2F> = MaybeUninit::zeroed();
		let mut tensor_point22_out_: MaybeUninit<D2DPoint2F> = MaybeUninit::zeroed();
		let _ret_ = D2D1GetGradientMeshInteriorPointsFromCoonsPatch(point0, point1, point2, point3, point4, point5, point6, point7, point8, point9, point10, point11, tensor_point11_out_.as_mut_ptr(), tensor_point12_out_.as_mut_ptr(), tensor_point21_out_.as_mut_ptr(), tensor_point22_out_.as_mut_ptr());
		(tensor_point11_out_.assume_init(), tensor_point12_out_.assume_init(), tensor_point21_out_.assume_init(), tensor_point22_out_.assume_init())
	}
}


pub type PD2d1EffectFactory 
	= unsafe extern "system" fn(effect_impl: &mut Unknown, ) -> HResult;

pub type PD2d1PropertySetFunction 
	= unsafe extern "system" fn(effect: Unknown, data: &u8, data_size: u32, ) -> HResult;

pub type PD2d1PropertyGetFunction 
	= unsafe extern "system" fn(effect: Unknown, data: Option<&mut u8>, data_size: u32, actual_size: Option<&mut u32>, ) -> HResult;

