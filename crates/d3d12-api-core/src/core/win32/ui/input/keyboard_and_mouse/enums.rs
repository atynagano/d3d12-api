#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HotKeyModifiers
{
	Alt                  = 0x1u32,
	Control              = 0x2u32,
	NoRepeat             = 0x4000u32,
	Shift                = 0x4u32,
	Win                  = 0x8u32,
}

impl BitOr for HotKeyModifiers {
	type Output = HotKeyModifiers;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for HotKeyModifiers {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl HotKeyModifiers {
    pub fn contains(self, other: HotKeyModifiers) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ActivateKeyboardLayoutFlags
{
	Reorder              = 0x8u32,
	Reset                = 0x40000000u32,
	SetForProcess        = 0x100u32,
	ShiftLock            = 0x10000u32,
	Activate             = 0x1u32,
	NoTellShell          = 0x80u32,
	ReplaceLang          = 0x10u32,
	SubstituteOk         = 0x2u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GetMouseMovePointsExResolution
{
	UseDisplayPoints     = 0x1u32,
	UseHighResolutionPoints = 0x2u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyBdEventFlags
{
	ExtendedKey          = 0x1u32,
	KeyUp                = 0x2u32,
	ScanCode             = 0x8u32,
	Unicode              = 0x4u32,
}

impl BitOr for KeyBdEventFlags {
	type Output = KeyBdEventFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for KeyBdEventFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl KeyBdEventFlags {
    pub fn contains(self, other: KeyBdEventFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseEventFlags
{
	Absolute             = 0x8000u32,
	LeftDown             = 0x2u32,
	LEfTup               = 0x4u32,
	MiddleDown           = 0x20u32,
	MiddleUp             = 0x40u32,
	Move                 = 0x1u32,
	RightDown            = 0x8u32,
	RigHTup              = 0x10u32,
	Wheel                = 0x800u32,
	XDown                = 0x80u32,
	XUp                  = 0x100u32,
	HWheel               = 0x1000u32,
	MoveNoCoalesce       = 0x2000u32,
	VirtualDesk          = 0x4000u32,
}

impl BitOr for MouseEventFlags {
	type Output = MouseEventFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for MouseEventFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl MouseEventFlags {
    pub fn contains(self, other: MouseEventFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputType
{
	Mouse                = 0x0u32,
	Keyboard             = 0x1u32,
	Hardware             = 0x2u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrackMouseEventFlags
{
	Cancel               = 0x80000000u32,
	Hover                = 0x1u32,
	Leave                = 0x2u32,
	NonClient            = 0x10u32,
	Query                = 0x40000000u32,
}

impl BitOr for TrackMouseEventFlags {
	type Output = TrackMouseEventFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for TrackMouseEventFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl TrackMouseEventFlags {
    pub fn contains(self, other: TrackMouseEventFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VirtualKey
{
	_0                   = 0x30u16,
	_1                   = 0x31u16,
	_2                   = 0x32u16,
	_3                   = 0x33u16,
	_4                   = 0x34u16,
	_5                   = 0x35u16,
	_6                   = 0x36u16,
	_7                   = 0x37u16,
	_8                   = 0x38u16,
	_9                   = 0x39u16,
	A                    = 0x41u16,
	B                    = 0x42u16,
	C                    = 0x43u16,
	D                    = 0x44u16,
	E                    = 0x45u16,
	F                    = 0x46u16,
	G                    = 0x47u16,
	H                    = 0x48u16,
	I                    = 0x49u16,
	J                    = 0x4Au16,
	K                    = 0x4Bu16,
	L                    = 0x4Cu16,
	M                    = 0x4Du16,
	N                    = 0x4Eu16,
	O                    = 0x4Fu16,
	P                    = 0x50u16,
	Q                    = 0x51u16,
	R                    = 0x52u16,
	S                    = 0x53u16,
	T                    = 0x54u16,
	U                    = 0x55u16,
	V                    = 0x56u16,
	W                    = 0x57u16,
	X                    = 0x58u16,
	Y                    = 0x59u16,
	Z                    = 0x5Au16,
	LButton              = 0x1u16,
	RButton              = 0x2u16,
	Cancel               = 0x3u16,
	MButton              = 0x4u16,
	XButton1             = 0x5u16,
	XButton2             = 0x6u16,
	Back                 = 0x8u16,
	Tab                  = 0x9u16,
	Clear                = 0xCu16,
	Return               = 0xDu16,
	Shift                = 0x10u16,
	Control              = 0x11u16,
	Menu                 = 0x12u16,
	Pause                = 0x13u16,
	Capital              = 0x14u16,
	Kana                 = 0x15u16,
	ImeOn                = 0x16u16,
	JunJa                = 0x17u16,
	Final                = 0x18u16,
	HanJa                = 0x19u16,
	ImeOff               = 0x1Au16,
	Escape               = 0x1Bu16,
	Convert              = 0x1Cu16,
	NonConvert           = 0x1Du16,
	Accept               = 0x1Eu16,
	ModeChange           = 0x1Fu16,
	Space                = 0x20u16,
	Prior                = 0x21u16,
	Next                 = 0x22u16,
	End                  = 0x23u16,
	Home                 = 0x24u16,
	Left                 = 0x25u16,
	Up                   = 0x26u16,
	Right                = 0x27u16,
	Down                 = 0x28u16,
	Select               = 0x29u16,
	Print                = 0x2Au16,
	Execute              = 0x2Bu16,
	Snapshot             = 0x2Cu16,
	Insert               = 0x2Du16,
	Delete               = 0x2Eu16,
	Help                 = 0x2Fu16,
	LWin                 = 0x5Bu16,
	RWin                 = 0x5Cu16,
	APps                 = 0x5Du16,
	Sleep                = 0x5Fu16,
	NumPad0              = 0x60u16,
	NumPad1              = 0x61u16,
	NumPad2              = 0x62u16,
	NumPad3              = 0x63u16,
	NumPad4              = 0x64u16,
	NumPad5              = 0x65u16,
	NumPad6              = 0x66u16,
	NumPad7              = 0x67u16,
	NumPad8              = 0x68u16,
	NumPad9              = 0x69u16,
	Multiply             = 0x6Au16,
	Add                  = 0x6Bu16,
	Separator            = 0x6Cu16,
	Subtract             = 0x6Du16,
	Decimal              = 0x6Eu16,
	Divide               = 0x6Fu16,
	F1                   = 0x70u16,
	F2                   = 0x71u16,
	F3                   = 0x72u16,
	F4                   = 0x73u16,
	F5                   = 0x74u16,
	F6                   = 0x75u16,
	F7                   = 0x76u16,
	F8                   = 0x77u16,
	F9                   = 0x78u16,
	F10                  = 0x79u16,
	F11                  = 0x7Au16,
	F12                  = 0x7Bu16,
	F13                  = 0x7Cu16,
	F14                  = 0x7Du16,
	F15                  = 0x7Eu16,
	F16                  = 0x7Fu16,
	F17                  = 0x80u16,
	F18                  = 0x81u16,
	F19                  = 0x82u16,
	F20                  = 0x83u16,
	F21                  = 0x84u16,
	F22                  = 0x85u16,
	F23                  = 0x86u16,
	F24                  = 0x87u16,
	NavigationView       = 0x88u16,
	NavigationMenu       = 0x89u16,
	NavigationUp         = 0x8Au16,
	NavigationDown       = 0x8Bu16,
	NavigationLeft       = 0x8Cu16,
	NavigationRight      = 0x8Du16,
	NavigationAccept     = 0x8Eu16,
	NavigationCancel     = 0x8Fu16,
	NumLock              = 0x90u16,
	Scroll               = 0x91u16,
	OEmNEcEqual          = 0x92u16,
	OEmFJMasShou         = 0x93u16,
	OEmFJTouRokU         = 0x94u16,
	OEmFJLoyA            = 0x95u16,
	OEmFJRoyA            = 0x96u16,
	LShift               = 0xA0u16,
	RShift               = 0xA1u16,
	LControl             = 0xA2u16,
	RControl             = 0xA3u16,
	LMenu                = 0xA4u16,
	RMenu                = 0xA5u16,
	BrowserBack          = 0xA6u16,
	BrowserForward       = 0xA7u16,
	BrowserRefresh       = 0xA8u16,
	BrowserStop          = 0xA9u16,
	BrowserSearch        = 0xAAu16,
	BrowserFavorites     = 0xABu16,
	BrowserHome          = 0xACu16,
	VolumeMute           = 0xADu16,
	VolumeDown           = 0xAEu16,
	VolumeUp             = 0xAFu16,
	MediaNextTrack       = 0xB0u16,
	MediaPrevTrack       = 0xB1u16,
	MediaStop            = 0xB2u16,
	MediaPlayPause       = 0xB3u16,
	LaunchMail           = 0xB4u16,
	LaunchMediaSelect    = 0xB5u16,
	LaunchApp1           = 0xB6u16,
	LaunchApp2           = 0xB7u16,
	OEm1                 = 0xBAu16,
	OEmPlus              = 0xBBu16,
	OEmComma             = 0xBCu16,
	OEmMinus             = 0xBDu16,
	OEmPeriod            = 0xBEu16,
	OEm2                 = 0xBFu16,
	OEm3                 = 0xC0u16,
	GamePadA             = 0xC3u16,
	GamePadB             = 0xC4u16,
	GamePadX             = 0xC5u16,
	GamePadY             = 0xC6u16,
	GamePadRightShoulder = 0xC7u16,
	GamePadLeftShoulder  = 0xC8u16,
	GamePadLeftTrigger   = 0xC9u16,
	GamePadRightTrigger  = 0xCAu16,
	GamePadDPadUp        = 0xCBu16,
	GamePadDPadDown      = 0xCCu16,
	GamePadDPadLeft      = 0xCDu16,
	GamePadDPadRight     = 0xCEu16,
	GamePadMenu          = 0xCFu16,
	GamePadView          = 0xD0u16,
	GamePadLeftThumbStickButton = 0xD1u16,
	GamePadRightThumbStickButton = 0xD2u16,
	GamePadLeftThumbStickUp = 0xD3u16,
	GamePadLeftThumbStickDown = 0xD4u16,
	GamePadLeftThumbStickRight = 0xD5u16,
	GamePadLeftThumbStickLeft = 0xD6u16,
	GamePadRightThumbStickUp = 0xD7u16,
	GamePadRightThumbStickDown = 0xD8u16,
	GamePadRightThumbStickRight = 0xD9u16,
	GamePadRightThumbStickLeft = 0xDAu16,
	OEm4                 = 0xDBu16,
	OEm5                 = 0xDCu16,
	OEm6                 = 0xDDu16,
	OEm7                 = 0xDEu16,
	OEm8                 = 0xDFu16,
	OEmAx                = 0xE1u16,
	OEm102               = 0xE2u16,
	ICoHelp              = 0xE3u16,
	ICo00                = 0xE4u16,
	ProcessKey           = 0xE5u16,
	ICoClear             = 0xE6u16,
	Packet               = 0xE7u16,
	OEmReset             = 0xE9u16,
	OEmJump              = 0xEAu16,
	OEmPa1               = 0xEBu16,
	OEmPa2               = 0xECu16,
	OEmPa3               = 0xEDu16,
	OEmWsCtrl            = 0xEEu16,
	OEmCuSel             = 0xEFu16,
	OEmAttn              = 0xF0u16,
	OEmFinish            = 0xF1u16,
	OEmCopy              = 0xF2u16,
	OEmAuto              = 0xF3u16,
	OEmEnlW              = 0xF4u16,
	OEmBackTab           = 0xF5u16,
	Attn                 = 0xF6u16,
	CrSel                = 0xF7u16,
	ExSel                = 0xF8u16,
	ErEof                = 0xF9u16,
	Play                 = 0xFAu16,
	Zoom                 = 0xFBu16,
	NoName               = 0xFCu16,
	Pa1                  = 0xFDu16,
	OEmClear             = 0xFEu16,
}

impl VirtualKey {
	pub const HangEuL             : Self = unsafe { transmute(0x15u16) };
	pub const Hangul              : Self = unsafe { transmute(0x15u16) };
	pub const Kanji               : Self = unsafe { transmute(0x19u16) };
	pub const OEmFJJiSho          : Self = unsafe { transmute(0x92u16) };
}

