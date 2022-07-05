#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PeekMessageRemoveType
{
	NoRemove             = 0x0u32,
	Remove               = 0x1u32,
	NoYield              = 0x2u32,
	QsInput              = 0x4070000u32,
	QsPostMessage        = 0x980000u32,
	QsPaint              = 0x200000u32,
	QsSendMessage        = 0x400000u32,
}

impl BitOr for PeekMessageRemoveType {
	type Output = PeekMessageRemoveType;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for PeekMessageRemoveType {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl PeekMessageRemoveType {
    pub fn contains(self, other: PeekMessageRemoveType) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WndClassStyles
{
	VRedraw              = 0x1u32,
	HRedraw              = 0x2u32,
	DblClks              = 0x8u32,
	OwnDc                = 0x20u32,
	ClassDc              = 0x40u32,
	ParentDc             = 0x80u32,
	NoClose              = 0x200u32,
	SaveBits             = 0x800u32,
	ByteAlignClient      = 0x1000u32,
	ByteAlignWindow      = 0x2000u32,
	GlobalClass          = 0x4000u32,
	Ime                  = 0x10000u32,
	DropShadow           = 0x20000u32,
}

impl BitOr for WndClassStyles {
	type Output = WndClassStyles;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for WndClassStyles {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl WndClassStyles {
    pub fn contains(self, other: WndClassStyles) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowExStyle
{
	DlgModalFrame        = 0x1u32,
	NoParentNotify       = 0x4u32,
	Topmost              = 0x8u32,
	AcceptFiles          = 0x10u32,
	Transparent          = 0x20u32,
	MdiChild             = 0x40u32,
	ToolWindow           = 0x80u32,
	WindowEdge           = 0x100u32,
	CLienTedge           = 0x200u32,
	ContextHelp          = 0x400u32,
	Right                = 0x1000u32,
	Left                 = 0x0u32,
	RtlReading           = 0x2000u32,
	LeftScRollbar        = 0x4000u32,
	ControlParent        = 0x10000u32,
	StaticEdge           = 0x20000u32,
	AppWindow            = 0x40000u32,
	OverlappedWindow     = 0x300u32,
	PaletteWindow        = 0x188u32,
	Layered              = 0x80000u32,
	NoInheritLayout      = 0x100000u32,
	NoRedirectionBitmap  = 0x200000u32,
	LayoutRtl            = 0x400000u32,
	Composited           = 0x2000000u32,
	NoActivate           = 0x8000000u32,
}

impl WindowExStyle {
	pub const LtrReading          : Self = unsafe { transmute(0x0u32) };
	pub const RightScRollbar      : Self = unsafe { transmute(0x0u32) };
}

impl BitOr for WindowExStyle {
	type Output = WindowExStyle;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for WindowExStyle {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl WindowExStyle {
    pub fn contains(self, other: WindowExStyle) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowStyle
{
	Overlapped           = 0x0u32,
	Popup                = 0x80000000u32,
	Child                = 0x40000000u32,
	Minimize             = 0x20000000u32,
	Visible              = 0x10000000u32,
	Disabled             = 0x8000000u32,
	ClipSiblings         = 0x4000000u32,
	ClipChildren         = 0x2000000u32,
	Maximize             = 0x1000000u32,
	Caption              = 0xC00000u32,
	Border               = 0x800000u32,
	DlgFrame             = 0x400000u32,
	VScroll              = 0x200000u32,
	HScroll              = 0x100000u32,
	SysMenu              = 0x80000u32,
	ThickFrame           = 0x40000u32,
	Group                = 0x20000u32,
	TabStop              = 0x10000u32,
	TiledWindow          = 0xCF0000u32,
	PopupWindow          = 0x80880000u32,
	ActiveCaption        = 0x1u32,
}

impl WindowStyle {
	pub const MinimizeBox         : Self = unsafe { transmute(0x20000u32) };
	pub const MaximizeBox         : Self = unsafe { transmute(0x10000u32) };
	pub const Tiled               : Self = unsafe { transmute(0x0u32) };
	pub const Iconic              : Self = unsafe { transmute(0x20000000u32) };
	pub const SizeBox             : Self = unsafe { transmute(0x40000u32) };
	pub const OverlappedWindow    : Self = unsafe { transmute(0xCF0000u32) };
	pub const ChildWindow         : Self = unsafe { transmute(0x40000000u32) };
}

impl BitOr for WindowStyle {
	type Output = WindowStyle;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for WindowStyle {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl WindowStyle {
    pub fn contains(self, other: WindowStyle) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShowWindowCmd
{
	ForceMinimize        = 0xBu32,
	Hide                 = 0x0u32,
	Maximize             = 0x3u32,
	Minimize             = 0x6u32,
	Restore              = 0x9u32,
	Show                 = 0x5u32,
	ShowDefault          = 0xAu32,
	ShowMinimized        = 0x2u32,
	ShowMinNoActive      = 0x7u32,
	ShowNa               = 0x8u32,
	ShowNoActivate       = 0x4u32,
	ShowNormal           = 0x1u32,
	SmoothScroll         = 0x10u32,
}

impl ShowWindowCmd {
	pub const ShowMaximized       : Self = unsafe { transmute(0x3u32) };
	pub const Normal              : Self = unsafe { transmute(0x1u32) };
	pub const Max                 : Self = unsafe { transmute(0xBu32) };
	pub const ParentClosing       : Self = unsafe { transmute(0x1u32) };
	pub const OtherZoom           : Self = unsafe { transmute(0x2u32) };
	pub const ParentOpening       : Self = unsafe { transmute(0x3u32) };
	pub const OTheRunZoom         : Self = unsafe { transmute(0x4u32) };
	pub const ScrollChildren      : Self = unsafe { transmute(0x1u32) };
	pub const Invalidate          : Self = unsafe { transmute(0x2u32) };
	pub const Erase               : Self = unsafe { transmute(0x4u32) };
}

impl BitOr for ShowWindowCmd {
	type Output = ShowWindowCmd;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for ShowWindowCmd {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl ShowWindowCmd {
    pub fn contains(self, other: ShowWindowCmd) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowLongPtrIndex
{
	ExStyle              = -0x14i32,
	HInstance            = -0x6i32,
	HWndParent           = -0x8i32,
	Id                   = -0xCi32,
	Style                = -0x10i32,
	UserData             = -0x15i32,
	WndProc              = -0x4i32,
}

