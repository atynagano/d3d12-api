/*mod IUnknown;
pub use self::IUnknown::*;*/
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use std::marker::PhantomData;
use std::mem::{MaybeUninit, transmute, transmute_copy};
use std::ops::Index;
use std::ptr::{NonNull, null};
use crate::helpers::Zeroed;
//use crate::Zeroed;
use super::super::super::foundation::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GUID {
    pub const fn from_u128(uuid: u128) -> GUID {
        GUID {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}

impl Debug for GUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}-{:04X}-{:04X}-{:016X}", self.data1, self.data2, self.data3, unsafe { transmute::<[u8; 8], u64>(self.data4) })
    }
}

pub trait Guid {
    const IID: &'static GUID;
}

pub trait Clsid {
    const CLSID: &'static GUID;
}

pub trait Com: Guid {
    fn vtable(&self) -> VTable;
}

pub(crate) fn option_to_vtable(a: Option<&impl Com>) -> *const c_void {
    if let Some(o) = a {
        unsafe { transmute(o.vtable()) }
    } else {
        null()
    }
}

impl<T: IUnknown> Com for T {
    fn vtable(&self) -> VTable {
        self.as_unknown().0
    }
}

#[repr(C)]
pub struct Unknown(VTable);

impl Clone for Unknown {
    // note: AddRef
    fn clone(&self) -> Self {
        let vt = self.vtable();
        let f: extern "system" fn(VTable) -> u32
            = unsafe { transmute(vt[1]) };
        f(vt);
        Unknown(self.0)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl Drop for Unknown {
    // note: Release
    fn drop(&mut self) {
        let vt = self.vtable();
        let f: extern "system" fn(VTable) -> u32
            = unsafe { transmute(vt[2]) };
        f(vt);
    }
}

impl Guid for Unknown {
    const IID: &'static GUID = &GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl IUnknown for Unknown {
    fn as_unknown(&self) -> &Unknown { self }
    fn into_unknown(self) -> Unknown { self }
}

pub trait IUnknown: Com + Clone + From<Unknown> {
    fn as_unknown(&self) -> &Unknown;
    fn into_unknown(self) -> Unknown;

    fn QueryInterface<T: IUnknown>(&self) -> Result<T, HResult> {
        let vt = self.as_param();
        let mut out: Option<Unknown> = None;
        let f: extern "system" fn(Param<Self>, *const GUID, &mut Option<Unknown>) -> HResult
            = unsafe { transmute(vt[0]) };
        let ret = f(vt, T::IID, &mut out);
        if ret.is_ok() {
            if let Some(o) = out {
                return Ok(T::from(o));
            }
        }
        Err(ret)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VTable(NonNull<*const *const c_void>);

impl Index<isize> for VTable {
    type Output = *const c_void;

    fn index(&self, index: isize) -> &Self::Output {
        unsafe {
            &*(*self.0.as_ptr()).offset(index)
        }
    }
}

impl<T: ?Sized> Index<isize> for Param<'_, T> {
    type Output = *const c_void;

    fn index(&self, index: isize) -> &Self::Output {
        self.0.index(index)
    }
}

// note: safely dereference borrow to value

#[repr(C)]
pub struct Param<'a, T: ?Sized>(pub(crate) VTable, PhantomData<&'a T>);

impl<'a, T: ?Sized> Clone for Param<'a, T> {
    fn clone(&self) -> Self {
        unsafe { transmute_copy(self) }
    }
}

impl<'a, T: ?Sized> Copy for Param<'a, T> {}

impl<'a, T: ?Sized> Debug for Param<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:016?}", self.0.0)
    }
}

pub trait AsParam {
    fn as_param(&self) -> Param<Self>;
}

impl<T: IUnknown> AsParam for T {
    fn as_param(&self) -> Param<T> {
        Param::<T>(self.vtable(), PhantomData)
    }
}

