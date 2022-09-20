use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use std::mem::{MaybeUninit, transmute};
use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, null};
use std::slice;
use crate::core::win32::foundation::{BStr, GetLastError, Rect, Win32Error};
use super::HResult;

impl Rect {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self { Self { left, top, right, bottom } }
    pub const fn with_size(left: i32, top: i32, width: u32, height: u32) -> Self {
        Self { left, top, right: left + width as i32, bottom: top + height as i32 }
    }
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
}

impl HResult {
    pub fn is_ok(self) -> bool {
        self as i32 >= 0
    }
    pub fn is_err(self) -> bool {
        (self as i32) < 0
    }
    pub fn ok(self) -> Result<(), HResult> {
        if self.is_ok() { Ok(()) } else { Err(self) }
    }
}

impl Display for HResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{:?}: ", *self)?;
        write_hresult(*self, f)
    }
}

impl Error for HResult {}

fn write_hresult(value: HResult, f: &mut Formatter<'_>) -> std::fmt::Result {
    unsafe {
        #[link(name = "KERNEL32")]
        extern "system" {
            fn FormatMessageA(
                dwFlags: u32,
                lpSource: isize,
                dwMessageId: HResult,
                dwLanguageId: u32,
                lpBuffer: *mut u8,
                nSize: u32,
                Arguments: isize) -> u32;
            fn LocalFree(hMem: *const u8) -> isize;
        }

        struct LocalPtr(NonNull<u8>);
        impl Drop for LocalPtr {
            fn drop(&mut self) {
                unsafe {
                    LocalFree(self.0.as_ptr());
                }
            }
        }

        // note:
        //  0x100 : FORMAT_MESSAGE_ALLOCATE_BUFFER
        //  0x200 : FORMAT_MESSAGE_IGNORE_INSERTS
        //  0x1000: FORMAT_MESSAGE_FROM_SYSTEM
        //  0x2000: FORMAT_MESSAGE_ARGUMENT_ARRAY
        let mut buffer = Box::new(MaybeUninit::<[u8; 256]>::uninit());
        let mut buf = buffer.assume_init_mut();
        // let mut buffer = MaybeUninit::<[u8; 256]>::uninit().assume_init();
        let length = FormatMessageA(0x0200 | 0x1000 | 0x2000, 0, value, 0, buf.as_mut_ptr(), buf.len() as u32, 0) - 1;
        if length > 0 {
            let s = slice::from_raw_parts(buf.as_ptr(), length as usize);
            let s: &str = transmute(s);
            return f.write_str(s);
        }
        if GetLastError() == Win32Error::InsufficientBuffer {
            let mut ptr: Option<LocalPtr> = None;
            let length = FormatMessageA(0x0200 | 0x1000 | 0x2000 | 0x100, 0, value, 0, transmute(&mut ptr), 0, 0) - 1;
            if length > 0 {
                if let Some(ptr) = ptr {
                    let s = slice::from_raw_parts(ptr.0.as_ptr(), length as usize);
                    let s: &str = transmute(s);
                    return f.write_str(s);
                }
            }
        }

        f.write_str("Description not found.")
    }
}

// todo: enumでは？
pub const FACILITY_DISPATCH: u32 = 2;
pub const FACILITY_ITF: u32 = 4;
pub const FACILITY_NULL: u32 = 0;
pub const FACILITY_RPC: u32 = 1;
pub const FACILITY_STORAGE: u32 = 3;
pub const FACILITY_WIN32: u32 = 7;
pub const FACILITY_WINDOWS: u32 = 8;

impl From<Win32Error> for HResult {
    fn from(x: Win32Error) -> Self {
        let x = x as u32;
        let result = if x <= 0 { x } else { (x & 0x0000FFFF) | (FACILITY_WIN32 << 16) | 0x80000000 };
        unsafe { transmute(result) }
    }
}

impl Display for Win32Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { write_hresult(transmute(*self), f) }
    }
}

impl Error for Win32Error {}

pub trait OkOrErr<T> {
    fn ok_or_err(self) -> Result<T, HResult>;
}

impl<T> OkOrErr<T> for Option<T> {
    fn ok_or_err(self) -> Result<T, HResult> {
        match self {
            Some(v) => Ok(v),
            None => Err(HResult::E_FAIL),
        }
    }
}

pub trait OkOrLastErr<T> {
    fn ok_or_last_err(self) -> Result<T, Win32Error>;
}

impl<T> OkOrLastErr<T> for Option<T> {
    fn ok_or_last_err(self) -> Result<T, Win32Error> {
        match self {
            Some(v) => Ok(v),
            None => Err(GetLastError()),
        }
    }
}


#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Bool(pub i32);

impl Bool {
    pub const TRUE: Self = Self(1);
    pub const FALSE: Self = Self(0);
    pub const fn to_bool(self) -> bool { self.0 != 0 }
}

pub trait ToBool {
    fn to_bool(self) -> Bool;
}

impl ToBool for bool {
    fn to_bool(self) -> Bool { Bool(self as i32) }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        value.to_bool()
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self.to_bool(), f) }
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(&self.to_bool(), f) }
}


#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PWStr<'a>(pub &'a u16);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PStr<'a>(pub &'a u8);

extern "C" {
    fn wcslen(buf: *const u16) -> usize;
    fn strlen(cs: *const u8) -> usize;
}

impl PWStr<'_> {
    fn len(&self) -> usize { unsafe { wcslen(self.0) } }
}

impl PStr<'_> {
    fn len(&self) -> usize { unsafe { strlen(self.0) } }
}

impl Debug for PWStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 as *const u16 == null() {
            return Ok(());
        }
        let text = String::from_utf16_lossy(unsafe { slice::from_raw_parts(self.0, self.len()) });
        Debug::fmt(&*text, f)
    }
}

impl Display for PWStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 as *const u16 == null() {
            return Ok(());
        }
        let text = String::from_utf16_lossy(unsafe { slice::from_raw_parts(self.0, self.len()) });
        Display::fmt(&*text, f)
    }
}

impl Debug for PStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 as *const u8 == null() {
            return write!(f, "");
        }
        let text = String::from_utf8_lossy(unsafe { slice::from_raw_parts(self.0, self.len()) });
        Debug::fmt(&*text, f)
    }
}

impl Display for PStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 as *const u8 == null() {
            return write!(f, "");
        }
        let text = String::from_utf8_lossy(unsafe { slice::from_raw_parts(self.0, self.len()) });
        Display::fmt(&*text, f)
    }
}

pub trait AsPWStr {
    fn as_pwstr(&self) -> Option<PWStr>;
}

impl AsPWStr for [u16] {
    fn as_pwstr(&self) -> Option<PWStr> {
        if *self.last()? != 0u16 { return None; }
        unsafe { Some(PWStr(&*self.as_ptr())) }
    }
}

pub trait AsPStr {
    fn as_pstr(&self) -> Option<PStr>;
}

impl AsPStr for str {
    fn as_pstr(&self) -> Option<PStr> {
        if *self.as_bytes().last()? != 0u8 { return None; }
        unsafe { Some(PStr(&*self.as_ptr())) }
    }
}


#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct WParam(pub usize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct LParam(pub usize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct LResult(pub usize);

impl LResult {
    pub fn new(value: usize) -> Self { Self(value) }
}

impl<T: Into<usize>> From<T> for LResult {
    fn from(value: T) -> Self { Self::new(value.into()) }
}

/* todo: BStrフィールドは存在しないのでpub(crate)にし引数なら&str、と戻り値ならStringに変換する
void SysFreeString(
  [in, optional] BSTR bstrString
);
impl Drop for BStr<'_>{
    fn drop(&mut self) {
        todo!()
    }
}
*/