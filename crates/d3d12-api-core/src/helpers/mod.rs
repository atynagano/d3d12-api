use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::mem::{MaybeUninit, transmute};
use std::ops::Deref;
use std::ptr::{null, null_mut};
use crate::core::win32::foundation::{AsPStr, AsPWStr, PStr, PWStr};

pub(crate) trait Len {
    fn len(&self) -> usize;
}

impl<T> Len for Option<&[T]> {
    fn len(&self) -> usize {
        if let Some(v) = self {
            v.len()
        } else {
            0
        }
    }
}

// todo: mutにすべきではない。Vecで返すべきだ。
impl<T> Len for Option<&mut [T]> {
    fn len(&self) -> usize {
        if let Some(v) = self {
            v.len()
        } else {
            0
        }
    }
}


pub(crate) trait Deconstruct<T> {
    fn deconstruct(self) -> T;
}

impl<T> Deconstruct<(*const T, usize)> for &[T] {
    fn deconstruct(self) -> (*const T, usize) {
        if self.len() > 0 {
            return (self.as_ptr(), self.len());
        }
        (null(), 0)
    }
}

impl<T> Deconstruct<(*mut T, usize)> for &mut [T] {
    fn deconstruct(self) -> (*mut T, usize) {
        if self.len() > 0 {
            return (self.as_mut_ptr(), self.len());
        }
        (null_mut(), 0)
    }
}

impl<T> Deconstruct<(*const T, usize)> for Option<&[T]> {
    fn deconstruct(self) -> (*const T, usize) {
        if let Some(v) = self {
            if v.len() > 0 {
                return (v.as_ptr(), v.len());
            }
        }
        (null(), 0)
    }
}

impl Deconstruct<(*const u8, usize)> for &str {
    fn deconstruct(self) -> (*const u8, usize) {
        if self.len() > 0 {
            return (self.as_ptr(), self.len());
        }
        (null(), 0)
    }
}

impl Deconstruct<(*const u8, usize)> for Option<&str> {
    fn deconstruct(self) -> (*const u8, usize) {
        if let Some(v) = self {
            if v.len() > 0 {
                return (v.as_ptr(), v.len());
            }
        }
        (null(), 0)
    }
}

pub fn low_high_word(v: u32) -> (u16, u16) {
    (v as u16 & 0xFFFFu16, ((v >> 16) as u16) & 0xFFFFu16)
}

pub enum Utf16Vec {
    Some(Vec<Utf16<'static>>, Vec<PWStr<'static>>),
    None,
}

impl Utf16Vec {
    pub(crate) fn ptr(&self) -> *const PWStr<'static> {
        match self {
            Utf16Vec::Some(_, v) => unsafe { transmute(v.as_ptr()) }
            Utf16Vec::None => null()
        }
    }
}

pub trait ToUtf16Vec {
    fn to_utf16_vec(&self) -> Utf16Vec;
}

impl ToUtf16Vec for [&str] {
    fn to_utf16_vec(&self) -> Utf16Vec {
        if self.len() > 0 {
            let a = self.into_iter().map(|v| v.to_utf16()).collect::<Vec<_>>();
            let b = a.iter().map(|w| w.as_pwstr()).collect::<Vec<_>>();
            let b = unsafe { transmute(b) };
            let a = unsafe { transmute(a) };
            Utf16Vec::Some(a, b)
        } else {
            Utf16Vec::None
        }
    }
}

impl ToUtf16Vec for Option<&[&str]> {
    fn to_utf16_vec(&self) -> Utf16Vec {
        if let Some(x) = self {
            if self.len() > 0 {
                let a = x.into_iter().map(|v| v.to_utf16()).collect::<Vec<_>>();
                let b = a.iter().map(|w| w.as_pwstr()).collect::<Vec<_>>();
                let b = unsafe { transmute(b) };
                let a = unsafe { transmute(a) };
                return Utf16Vec::Some(a, b);
            }
        }
        Utf16Vec::None
    }
}

pub enum NullTerminatedVec<'a> {
    Some(Vec<NullTerminated<'a>>, Vec<PStr<'a>>),
    None,
}

impl NullTerminatedVec<'_> {
    pub fn ptr(&self) -> *const PStr {
        match self {
            NullTerminatedVec::Some(_, v) => unsafe { transmute(v.as_ptr()) }
            NullTerminatedVec::None => null()
        }
    }
}

pub trait ToNullTerminatedVec {
    fn to_null_terminated_vec(&self) -> NullTerminatedVec;
}

impl ToNullTerminatedVec for [&str] {
    fn to_null_terminated_vec(&self) -> NullTerminatedVec {
        if self.len() > 0 {
            let a = self.into_iter().map(|v| v.to_null_terminated()).collect::<Vec<_>>();
            let b = a.iter().map(|w| w.as_pstr()).collect::<Vec<_>>();
            let b = unsafe { transmute(b) };
            NullTerminatedVec::Some(a, b)
        } else {
            NullTerminatedVec::None
        }
    }
}

impl ToNullTerminatedVec for Option<&[&str]> {
    fn to_null_terminated_vec(&self) -> NullTerminatedVec {
        if let Some(x) = self {
            if self.len() > 0 {
                let a = x.into_iter().map(|v| v.to_null_terminated()).collect::<Vec<_>>();
                let b = a.iter().map(|w| w.as_pstr()).collect::<Vec<_>>();
                let b = unsafe { transmute(b) };
                return NullTerminatedVec::Some(a, b);
            }
        }
        NullTerminatedVec::None
    }
}

pub trait AsPtrOrNull<T> {
    fn as_ptr_or_null(&self) -> *const T;
}

impl<T> AsPtrOrNull<T> for [T] {
    fn as_ptr_or_null(&self) -> *const T {
        if let Some(v) = self.first() {
            return v as *const T;
        }
        null()
    }
}

impl<T, const N: usize> AsPtrOrNull<T> for [T; N] {
    fn as_ptr_or_null(&self) -> *const T {
        if let Some(v) = self.first() {
            return v as *const T;
        }
        null()
    }
}

impl AsPtrOrNull<u8> for Option<&str> {
    fn as_ptr_or_null(&self) -> *const u8 {
        if let Some(v) = self {
            if let Some(w) = v.as_bytes().first() {
                return w as *const u8;
            }
        }
        null()
    }
}

impl AsPtrOrNull<u8> for str {
    fn as_ptr_or_null(&self) -> *const u8 {
        if let Some(v) = self.as_bytes().first() {
            return v as *const u8;
        }
        null()
    }
}

impl<T> AsPtrOrNull<T> for Option<*const T> {
    fn as_ptr_or_null(&self) -> *const T {
        if let Some(v) = self {
            return *v;
        }
        null()
    }
}

impl<T> AsPtrOrNull<T> for Option<&[T]> {
    fn as_ptr_or_null(&self) -> *const T {
        if let Some(v) = self {
            if let Some(w) = v.first() {
                return w as *const T;
            }
        }
        null()
    }
}

impl<T, const N: usize> AsPtrOrNull<T> for Option<&[T; N]> {
    fn as_ptr_or_null(&self) -> *const T {
        if let Some(v) = self {
            if let Some(w) = v.first() {
                return w as *const T;
            }
        }
        null()
    }
}

impl AsPtrOrNull<u8> for Option<NullTerminated<'_>> {
    fn as_ptr_or_null(&self) -> *const u8 {
        if let Some(v) = self {
            if let Some(w) = v.as_bytes().first() {
                return w as *const u8;
            }
        }
        null()
    }
}

impl AsPtrOrNull<u16> for Option<Utf16<'_>> {
    fn as_ptr_or_null(&self) -> *const u16 {
        if let Some(v) = self {
            if let Some(w) = v.first() {
                return w as *const u16;
            }
        }
        null()
    }
}

impl AsPtrOrNull<u16> for Option<&Utf16<'_>> {
    fn as_ptr_or_null(&self) -> *const u16 {
        if let Some(v) = self {
            if let Some(w) = v.first() {
                return w as *const u16;
            }
        }
        null()
    }
}

pub(crate) trait AsMutPtrOrNull<T> {
    fn as_mut_ptr_or_null(self) -> *mut T;
}

impl<T> AsMutPtrOrNull<T> for Option<&mut [T]> {
    fn as_mut_ptr_or_null(self) -> *mut T {
        if let Some(v) = self {
            if let Some(w) = v.first_mut() {
                return w as *mut T;
            }
        }
        null_mut()
    }
}

impl<T> AsMutPtrOrNull<T> for &mut [T] {
    fn as_mut_ptr_or_null(self) -> *mut T {
        if let Some(v) = self.first_mut() {
            return v as *mut T;
        }
        null_mut()
    }
}

pub struct NullTerminated<'a>(pub Cow<'a, str>);

impl NullTerminated<'_> {
    pub fn as_pstr(&self) -> PStr<'_> {
        unsafe { self.0.as_pstr().unwrap_unchecked() }
    }
}

/*
impl NullTerminated<'_> {
    pub fn len(&self) -> usize {
        self.cow.len() - 1
    }
}*/

impl Deref for NullTerminated<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Debug for NullTerminated<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for NullTerminated<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_pstr())
    }
}

pub trait ToNullTerminated {
    fn to_null_terminated(&self) -> NullTerminated;
}

impl ToNullTerminated for str {
    fn to_null_terminated(&self) -> NullTerminated {
        if let Some(&ter) = self.as_bytes().last() {
            if ter == 0 {
                return NullTerminated(Cow::Borrowed(self));
            }
        }

        NullTerminated(Cow::Owned(format!("{}\0", self)))
    }
}

pub struct Utf16<'a>(Cow<'a, [u16]>);

impl Utf16<'_> {
    pub fn as_pwstr(&self) -> PWStr<'_> {
        unsafe { self.0.as_pwstr().unwrap_unchecked() }
    }
}

/*
impl Utf16 {
    pub fn len(&self) -> usize {
        self.vec.len() - 1
    }
}*/

impl Deref for Utf16<'_> {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Debug for Utf16<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Utf16<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_pwstr())
    }
}

pub trait ToUtf16<'a, 'b> {
    fn to_utf16(&'a self) -> Utf16<'b>;
}

impl<'a> ToUtf16<'a, 'static> for str {
    fn to_utf16(&'a self) -> Utf16<'static> {
        Utf16(Cow::Owned(self.encode_utf16().chain([0u16]).collect::<Vec<u16>>()))
    }
}

impl<'a> ToUtf16<'a, 'a> for [u16] {
    fn to_utf16(&'a self) -> Utf16<'a> {
        if let Some(&ter) = self.last() {
            if ter == 0 {
                return Utf16(Cow::Borrowed(self));
            }
        }

        Utf16(Cow::Owned([self, &[0u16]].concat()))
    }
}

pub trait FillRestWithDefault<T: Default + Copy> {
    fn fill_rest_with_default<const N: usize>(&self) -> [T; N];
}

impl<T: Default + Copy> FillRestWithDefault<T> for [T] {
    fn fill_rest_with_default<const N: usize>(&self) -> [T; N] {
        fill_array_rest_with_default(self)
    }
}

// note: Copy and Drop cannot be implemented at the same time
pub fn fill_array_rest_with_default<T: Default + Copy, const N: usize>(head: &[T]) -> [T; N] {
    let len = head.len();
    assert!(N >= len);
    let mut result = MaybeUninit::<[T; N]>::uninit();
    let ptr = unsafe { &mut *result.as_mut_ptr() };
    for i in 0..len {
        ptr[i] = head[i];
    }
    for i in len..N {
        ptr[i] = T::default();
    }
    unsafe {
        result.assume_init()
    }
}

pub trait FillRestWith<T> {
    fn fill_rest_with<const N: usize>(&self, value: T) -> [T; N];
}

impl<T: Copy> FillRestWith<T> for [T] {
    fn fill_rest_with<const N: usize>(&self, value: T) -> [T; N] {
        fill_array_rest_with(self, value)
    }
}

pub fn fill_array_rest_with<T: Copy, const N: usize>(head: &[T], value: T) -> [T; N] {
    let len = head.len();
    assert!(N >= len);
    let mut result = MaybeUninit::<[T; N]>::uninit();
    let ptr = unsafe { &mut *result.as_mut_ptr() };
    for i in 0..len {
        ptr[i] = head[i];
    }
    for i in len..N {
        ptr[i] = value;
    }
    unsafe {
        result.assume_init()
    }
}

pub(crate) struct DebugWithSuffix<'a, T: Debug> {
    value: &'a T,
    suffix: &'a str,
}

impl<'a, T: Debug> DebugWithSuffix<'a, T> {
    pub(crate) fn new(value: &'a T, unit: &'a str) -> Self {
        Self {
            value,
            suffix: unit,
        }
    }
}

impl<T: Debug> Debug for DebugWithSuffix<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)?;
        f.write_str(self.suffix)
    }
}

pub(crate) struct DebugStringWithoutQuot<'a> {
    value: &'a str,
}

impl<'a> DebugStringWithoutQuot<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        Self {
            value,
        }
    }
}

impl Debug for DebugStringWithoutQuot<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value)
    }
}