use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use std::ptr::null;
use std::slice;
use crate::core::win32::foundation::Rect;
use crate::helpers::Uninit;
use super::HResult;

impl Rect {
    pub fn new(width: u32, height: u32) -> Self { Self { left: 0, top: 0, right: width as i32, bottom: height as i32 } }
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
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
        write!(f, "{}", hresult_to_string(*self))
    }
}

fn hresult_to_string(value: HResult) -> String {
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
            fn GetLastError() -> u32;
            fn LocalFree(hMem: *const u8) -> isize;
        }
        let mut ret = format!("{:?}: ", value);
        let mut buffer = <[u8; 256]>::uninit();
        let length = FormatMessageA(0x0200 | 0x1000 | 0x2000, 0, value, 0, buffer.as_mut_ptr(), buffer.len() as u32, 0) - 1;
        if length > 0 {
            let text = String::from_utf8_lossy(slice::from_raw_parts(buffer.as_ptr(), length as usize)).to_string();
            ret.push_str(text.as_str());
            return ret;
        }
        if GetLastError() == 122 {
            let mut ptr: *const u8 = null();
            let length = FormatMessageA(0x0200 | 0x1000 | 0x2000 | 0x100, 0, value, 0, transmute(&mut ptr), 0, 0) - 1;
            if length > 0 {
                let text = String::from_utf8_lossy(slice::from_raw_parts(ptr, length as usize)).to_string();
                ret.push_str(text.as_str());
                LocalFree(ptr);
                return ret;
            }
        }

        ret.push_str("Message not found.");
        ret
    }
}

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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bool {
    pub value: i32,
}

impl Bool {
    pub const TRUE: Self = Self { value: 1 };
    pub const FALSE: Self = Self { value: 0 };
    pub const fn to_bool(self) -> bool {
        self.value != 0
    }
}

pub trait ToBool {
    fn to_bool(self) -> Bool;
}

impl ToBool for bool {
    fn to_bool(self) -> Bool {
        Bool { value: self as i32 }
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        value.to_bool()
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_bool())
    }
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bool())
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct PWStr<'a> {
    pub value: &'a u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PStr<'a> {
    pub value: &'a u8,
}

extern "C" {
    fn wcslen(buf: *const u16) -> usize;
    fn strlen(cs: *const u8) -> usize;
}

impl PWStr<'_> {
    fn len(&self) -> usize {
        unsafe { wcslen(self.value as *const u16) }
    }
}

impl PStr<'_> {
    fn len(&self) -> usize {
        unsafe { strlen(self.value as *const u8) }
    }
}

impl Debug for PWStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value as *const u16 == null() {
            return write!(f, "");
        }
        let text = String::from_utf16_lossy(
            unsafe {
                slice::from_raw_parts(self.value as *const u16, self.len())
            }
        );
        write!(f, "{}", text)
    }
}

impl Display for PWStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Debug for PStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value as *const u8 == null() {
            return write!(f, "");
        }
        let text = String::from_utf8_lossy(
            unsafe {
                slice::from_raw_parts(self.value as *const u8, self.len())
            }
        );
        write!(f, "{}", text.to_string())
    }
}

impl Display for PStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

pub trait AsPWStr {
    fn as_pwstr(&self) -> PWStr;
}

impl AsPWStr for [u16] {
    fn as_pwstr(&self) -> PWStr {
        let last = *self.last().expect("PWStr must be longer than 0.");
        assert_eq!(last, 0u16, "PWStr must be null terminated.");
        unsafe {
            PWStr {
                value: &*self.as_ptr()
            }
        }
    }
}

pub trait AsPStr {
    fn as_pstr(&self) -> PStr;
}

impl AsPStr for str {
    fn as_pstr(&self) -> PStr {
        let last = *self.as_bytes().last().expect("PStr must be longer than 0.");
        assert_eq!(last, 0u8, "PStr must be null terminated.");
        unsafe {
            PStr {
                value: &*self.as_ptr()
            }
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WParam {
    pub value: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LParam {
    pub value: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LResult {
    pub value: usize,
}

impl LResult {
    pub fn new(value: impl Into<usize>) -> Self {
        Self {
            value: value.into()
        }
    }
}

impl<T: TryInto<usize>> From<T> for LResult {
    fn from(value: T) -> Self {
        Self {
            value: unsafe { value.try_into().unwrap_unchecked() }
        }
    }
}