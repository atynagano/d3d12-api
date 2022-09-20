#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

bitflags::bitflags! {
	/// HOT_KEY_MODIFIERS
	#[repr(transparent)]
	pub struct HotKeyModifiers: u32 {
		/// MOD_ALT = 0x1u32
		const Alt                  = 0x1u32;
		/// MOD_CONTROL = 0x2u32
		const Control              = 0x2u32;
		/// MOD_NOREPEAT = 0x4000u32
		const NoRepeat             = 0x4000u32;
		/// MOD_SHIFT = 0x4u32
		const Shift                = 0x4u32;
		/// MOD_WIN = 0x8u32
		const Win                  = 0x8u32;
	}
}

/// ACTIVATE_KEYBOARD_LAYOUT_FLAGS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivateKeyboardLayoutFlags
{
	/// KLF_REORDER = 0x8u32
	Reorder              = 0x8u32,
	/// KLF_RESET = 0x40000000u32
	Reset                = 0x40000000u32,
	/// KLF_SETFORPROCESS = 0x100u32
	SetForProcess        = 0x100u32,
	/// KLF_SHIFTLOCK = 0x10000u32
	ShiftLock            = 0x10000u32,
	/// KLF_ACTIVATE = 0x1u32
	Activate             = 0x1u32,
	/// KLF_NOTELLSHELL = 0x80u32
	NoTellShell          = 0x80u32,
	/// KLF_REPLACELANG = 0x10u32
	ReplaceLang          = 0x10u32,
	/// KLF_SUBSTITUTE_OK = 0x2u32
	SubstituteOk         = 0x2u32,
}

/// GET_MOUSE_MOVE_POINTS_EX_RESOLUTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetMouseMovePointsExResolution
{
	/// GMMP_USE_DISPLAY_POINTS = 0x1u32
	UseDisplayPoints     = 0x1u32,
	/// GMMP_USE_HIGH_RESOLUTION_POINTS = 0x2u32
	UseHighResolutionPoints = 0x2u32,
}

bitflags::bitflags! {
	/// KEYBD_EVENT_FLAGS
	#[repr(transparent)]
	pub struct KeyBDEventFlags: u32 {
		/// KEYEVENTF_EXTENDEDKEY = 0x1u32
		const ExtendedKey          = 0x1u32;
		/// KEYEVENTF_KEYUP = 0x2u32
		const KeyUp                = 0x2u32;
		/// KEYEVENTF_SCANCODE = 0x8u32
		const ScanCode             = 0x8u32;
		/// KEYEVENTF_UNICODE = 0x4u32
		const Unicode              = 0x4u32;
	}
}

bitflags::bitflags! {
	/// MOUSE_EVENT_FLAGS
	#[repr(transparent)]
	pub struct MouseEventFlags: u32 {
		/// MOUSEEVENTF_ABSOLUTE = 0x8000u32
		const Absolute             = 0x8000u32;
		/// MOUSEEVENTF_LEFTDOWN = 0x2u32
		const LeftDown             = 0x2u32;
		/// MOUSEEVENTF_LEFTUP = 0x4u32
		const LEFTup               = 0x4u32;
		/// MOUSEEVENTF_MIDDLEDOWN = 0x20u32
		const MiddleDown           = 0x20u32;
		/// MOUSEEVENTF_MIDDLEUP = 0x40u32
		const MiddleUp             = 0x40u32;
		/// MOUSEEVENTF_MOVE = 0x1u32
		const Move                 = 0x1u32;
		/// MOUSEEVENTF_RIGHTDOWN = 0x8u32
		const RightDown            = 0x8u32;
		/// MOUSEEVENTF_RIGHTUP = 0x10u32
		const RigHTup              = 0x10u32;
		/// MOUSEEVENTF_WHEEL = 0x800u32
		const Wheel                = 0x800u32;
		/// MOUSEEVENTF_XDOWN = 0x80u32
		const XDown                = 0x80u32;
		/// MOUSEEVENTF_XUP = 0x100u32
		const Xup                  = 0x100u32;
		/// MOUSEEVENTF_HWHEEL = 0x1000u32
		const HWheel               = 0x1000u32;
		/// MOUSEEVENTF_MOVE_NOCOALESCE = 0x2000u32
		const MoveNoCoalesce       = 0x2000u32;
		/// MOUSEEVENTF_VIRTUALDESK = 0x4000u32
		const VirtualDesk          = 0x4000u32;
	}
}

/// INPUT_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InputType
{
	/// INPUT_MOUSE = 0x0u32
	Mouse                = 0x0u32,
	/// INPUT_KEYBOARD = 0x1u32
	Keyboard             = 0x1u32,
	/// INPUT_HARDWARE = 0x2u32
	Hardware             = 0x2u32,
}

bitflags::bitflags! {
	/// TRACKMOUSEEVENT_FLAGS
	#[repr(transparent)]
	pub struct TrackMouseEventFlags: u32 {
		/// TME_CANCEL = 0x80000000u32
		const Cancel               = 0x80000000u32;
		/// TME_HOVER = 0x1u32
		const Hover                = 0x1u32;
		/// TME_LEAVE = 0x2u32
		const Leave                = 0x2u32;
		/// TME_NONCLIENT = 0x10u32
		const NonClient            = 0x10u32;
		/// TME_QUERY = 0x40000000u32
		const Query                = 0x40000000u32;
	}
}

/// VIRTUAL_KEY
#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VirtualKey
{
	/// VK_0 = 0x30u16
	_0                   = 0x30u16,
	/// VK_1 = 0x31u16
	_1                   = 0x31u16,
	/// VK_2 = 0x32u16
	_2                   = 0x32u16,
	/// VK_3 = 0x33u16
	_3                   = 0x33u16,
	/// VK_4 = 0x34u16
	_4                   = 0x34u16,
	/// VK_5 = 0x35u16
	_5                   = 0x35u16,
	/// VK_6 = 0x36u16
	_6                   = 0x36u16,
	/// VK_7 = 0x37u16
	_7                   = 0x37u16,
	/// VK_8 = 0x38u16
	_8                   = 0x38u16,
	/// VK_9 = 0x39u16
	_9                   = 0x39u16,
	/// VK_A = 0x41u16
	A                    = 0x41u16,
	/// VK_B = 0x42u16
	B                    = 0x42u16,
	/// VK_C = 0x43u16
	C                    = 0x43u16,
	/// VK_D = 0x44u16
	D                    = 0x44u16,
	/// VK_E = 0x45u16
	E                    = 0x45u16,
	/// VK_F = 0x46u16
	F                    = 0x46u16,
	/// VK_G = 0x47u16
	G                    = 0x47u16,
	/// VK_H = 0x48u16
	H                    = 0x48u16,
	/// VK_I = 0x49u16
	I                    = 0x49u16,
	/// VK_J = 0x4Au16
	J                    = 0x4Au16,
	/// VK_K = 0x4Bu16
	K                    = 0x4Bu16,
	/// VK_L = 0x4Cu16
	L                    = 0x4Cu16,
	/// VK_M = 0x4Du16
	M                    = 0x4Du16,
	/// VK_N = 0x4Eu16
	N                    = 0x4Eu16,
	/// VK_O = 0x4Fu16
	O                    = 0x4Fu16,
	/// VK_P = 0x50u16
	P                    = 0x50u16,
	/// VK_Q = 0x51u16
	Q                    = 0x51u16,
	/// VK_R = 0x52u16
	R                    = 0x52u16,
	/// VK_S = 0x53u16
	S                    = 0x53u16,
	/// VK_T = 0x54u16
	T                    = 0x54u16,
	/// VK_U = 0x55u16
	U                    = 0x55u16,
	/// VK_V = 0x56u16
	V                    = 0x56u16,
	/// VK_W = 0x57u16
	W                    = 0x57u16,
	/// VK_X = 0x58u16
	X                    = 0x58u16,
	/// VK_Y = 0x59u16
	Y                    = 0x59u16,
	/// VK_Z = 0x5Au16
	Z                    = 0x5Au16,
	/// VK_LBUTTON = 0x1u16
	LButton              = 0x1u16,
	/// VK_RBUTTON = 0x2u16
	RButton              = 0x2u16,
	/// VK_CANCEL = 0x3u16
	Cancel               = 0x3u16,
	/// VK_MBUTTON = 0x4u16
	MButton              = 0x4u16,
	/// VK_XBUTTON1 = 0x5u16
	XButton1             = 0x5u16,
	/// VK_XBUTTON2 = 0x6u16
	XButton2             = 0x6u16,
	/// VK_BACK = 0x8u16
	Back                 = 0x8u16,
	/// VK_TAB = 0x9u16
	Tab                  = 0x9u16,
	/// VK_CLEAR = 0xCu16
	Clear                = 0xCu16,
	/// VK_RETURN = 0xDu16
	Return               = 0xDu16,
	/// VK_SHIFT = 0x10u16
	Shift                = 0x10u16,
	/// VK_CONTROL = 0x11u16
	Control              = 0x11u16,
	/// VK_MENU = 0x12u16
	Menu                 = 0x12u16,
	/// VK_PAUSE = 0x13u16
	Pause                = 0x13u16,
	/// VK_CAPITAL = 0x14u16
	Capital              = 0x14u16,
	/// VK_KANA = 0x15u16
	Kana                 = 0x15u16,
	/// VK_IME_ON = 0x16u16
	ImeOn                = 0x16u16,
	/// VK_JUNJA = 0x17u16
	JunJA                = 0x17u16,
	/// VK_FINAL = 0x18u16
	Final                = 0x18u16,
	/// VK_HANJA = 0x19u16
	HanJA                = 0x19u16,
	/// VK_IME_OFF = 0x1Au16
	ImeOff               = 0x1Au16,
	/// VK_ESCAPE = 0x1Bu16
	Escape               = 0x1Bu16,
	/// VK_CONVERT = 0x1Cu16
	Convert              = 0x1Cu16,
	/// VK_NONCONVERT = 0x1Du16
	NonConvert           = 0x1Du16,
	/// VK_ACCEPT = 0x1Eu16
	Accept               = 0x1Eu16,
	/// VK_MODECHANGE = 0x1Fu16
	ModeChange           = 0x1Fu16,
	/// VK_SPACE = 0x20u16
	Space                = 0x20u16,
	/// VK_PRIOR = 0x21u16
	Prior                = 0x21u16,
	/// VK_NEXT = 0x22u16
	Next                 = 0x22u16,
	/// VK_END = 0x23u16
	End                  = 0x23u16,
	/// VK_HOME = 0x24u16
	Home                 = 0x24u16,
	/// VK_LEFT = 0x25u16
	Left                 = 0x25u16,
	/// VK_UP = 0x26u16
	Up                   = 0x26u16,
	/// VK_RIGHT = 0x27u16
	Right                = 0x27u16,
	/// VK_DOWN = 0x28u16
	Down                 = 0x28u16,
	/// VK_SELECT = 0x29u16
	Select               = 0x29u16,
	/// VK_PRINT = 0x2Au16
	Print                = 0x2Au16,
	/// VK_EXECUTE = 0x2Bu16
	Execute              = 0x2Bu16,
	/// VK_SNAPSHOT = 0x2Cu16
	Snapshot             = 0x2Cu16,
	/// VK_INSERT = 0x2Du16
	Insert               = 0x2Du16,
	/// VK_DELETE = 0x2Eu16
	Delete               = 0x2Eu16,
	/// VK_HELP = 0x2Fu16
	Help                 = 0x2Fu16,
	/// VK_LWIN = 0x5Bu16
	LWin                 = 0x5Bu16,
	/// VK_RWIN = 0x5Cu16
	RWin                 = 0x5Cu16,
	/// VK_APPS = 0x5Du16
	Apps                 = 0x5Du16,
	/// VK_SLEEP = 0x5Fu16
	Sleep                = 0x5Fu16,
	/// VK_NUMPAD0 = 0x60u16
	NumPad0              = 0x60u16,
	/// VK_NUMPAD1 = 0x61u16
	NumPad1              = 0x61u16,
	/// VK_NUMPAD2 = 0x62u16
	NumPad2              = 0x62u16,
	/// VK_NUMPAD3 = 0x63u16
	NumPad3              = 0x63u16,
	/// VK_NUMPAD4 = 0x64u16
	NumPad4              = 0x64u16,
	/// VK_NUMPAD5 = 0x65u16
	NumPad5              = 0x65u16,
	/// VK_NUMPAD6 = 0x66u16
	NumPad6              = 0x66u16,
	/// VK_NUMPAD7 = 0x67u16
	NumPad7              = 0x67u16,
	/// VK_NUMPAD8 = 0x68u16
	NumPad8              = 0x68u16,
	/// VK_NUMPAD9 = 0x69u16
	NumPad9              = 0x69u16,
	/// VK_MULTIPLY = 0x6Au16
	Multiply             = 0x6Au16,
	/// VK_ADD = 0x6Bu16
	Add                  = 0x6Bu16,
	/// VK_SEPARATOR = 0x6Cu16
	Separator            = 0x6Cu16,
	/// VK_SUBTRACT = 0x6Du16
	Subtract             = 0x6Du16,
	/// VK_DECIMAL = 0x6Eu16
	Decimal              = 0x6Eu16,
	/// VK_DIVIDE = 0x6Fu16
	Divide               = 0x6Fu16,
	/// VK_F1 = 0x70u16
	F1                   = 0x70u16,
	/// VK_F2 = 0x71u16
	F2                   = 0x71u16,
	/// VK_F3 = 0x72u16
	F3                   = 0x72u16,
	/// VK_F4 = 0x73u16
	F4                   = 0x73u16,
	/// VK_F5 = 0x74u16
	F5                   = 0x74u16,
	/// VK_F6 = 0x75u16
	F6                   = 0x75u16,
	/// VK_F7 = 0x76u16
	F7                   = 0x76u16,
	/// VK_F8 = 0x77u16
	F8                   = 0x77u16,
	/// VK_F9 = 0x78u16
	F9                   = 0x78u16,
	/// VK_F10 = 0x79u16
	F10                  = 0x79u16,
	/// VK_F11 = 0x7Au16
	F11                  = 0x7Au16,
	/// VK_F12 = 0x7Bu16
	F12                  = 0x7Bu16,
	/// VK_F13 = 0x7Cu16
	F13                  = 0x7Cu16,
	/// VK_F14 = 0x7Du16
	F14                  = 0x7Du16,
	/// VK_F15 = 0x7Eu16
	F15                  = 0x7Eu16,
	/// VK_F16 = 0x7Fu16
	F16                  = 0x7Fu16,
	/// VK_F17 = 0x80u16
	F17                  = 0x80u16,
	/// VK_F18 = 0x81u16
	F18                  = 0x81u16,
	/// VK_F19 = 0x82u16
	F19                  = 0x82u16,
	/// VK_F20 = 0x83u16
	F20                  = 0x83u16,
	/// VK_F21 = 0x84u16
	F21                  = 0x84u16,
	/// VK_F22 = 0x85u16
	F22                  = 0x85u16,
	/// VK_F23 = 0x86u16
	F23                  = 0x86u16,
	/// VK_F24 = 0x87u16
	F24                  = 0x87u16,
	/// VK_NAVIGATION_VIEW = 0x88u16
	NavigationView       = 0x88u16,
	/// VK_NAVIGATION_MENU = 0x89u16
	NavigationMenu       = 0x89u16,
	/// VK_NAVIGATION_UP = 0x8Au16
	NavigationUp         = 0x8Au16,
	/// VK_NAVIGATION_DOWN = 0x8Bu16
	NavigationDown       = 0x8Bu16,
	/// VK_NAVIGATION_LEFT = 0x8Cu16
	NavigationLeft       = 0x8Cu16,
	/// VK_NAVIGATION_RIGHT = 0x8Du16
	NavigationRight      = 0x8Du16,
	/// VK_NAVIGATION_ACCEPT = 0x8Eu16
	NavigationAccept     = 0x8Eu16,
	/// VK_NAVIGATION_CANCEL = 0x8Fu16
	NavigationCancel     = 0x8Fu16,
	/// VK_NUMLOCK = 0x90u16
	NumLock              = 0x90u16,
	/// VK_SCROLL = 0x91u16
	Scroll               = 0x91u16,
	/// VK_OEM_NEC_EQUAL = 0x92u16
	OemNecEqual          = 0x92u16,
	/// VK_OEM_FJ_MASSHOU = 0x93u16
	OemFjMasshou         = 0x93u16,
	/// VK_OEM_FJ_TOUROKU = 0x94u16
	OemFjTouroku         = 0x94u16,
	/// VK_OEM_FJ_LOYA = 0x95u16
	OemFjLoya            = 0x95u16,
	/// VK_OEM_FJ_ROYA = 0x96u16
	OemFjRoya            = 0x96u16,
	/// VK_LSHIFT = 0xA0u16
	LShift               = 0xA0u16,
	/// VK_RSHIFT = 0xA1u16
	RShift               = 0xA1u16,
	/// VK_LCONTROL = 0xA2u16
	LControl             = 0xA2u16,
	/// VK_RCONTROL = 0xA3u16
	RControl             = 0xA3u16,
	/// VK_LMENU = 0xA4u16
	LMenu                = 0xA4u16,
	/// VK_RMENU = 0xA5u16
	RMenu                = 0xA5u16,
	/// VK_BROWSER_BACK = 0xA6u16
	BrowserBack          = 0xA6u16,
	/// VK_BROWSER_FORWARD = 0xA7u16
	BrowserForward       = 0xA7u16,
	/// VK_BROWSER_REFRESH = 0xA8u16
	BrowserRefresh       = 0xA8u16,
	/// VK_BROWSER_STOP = 0xA9u16
	BrowserStop          = 0xA9u16,
	/// VK_BROWSER_SEARCH = 0xAAu16
	BrowserSearch        = 0xAAu16,
	/// VK_BROWSER_FAVORITES = 0xABu16
	BrowserFavorites     = 0xABu16,
	/// VK_BROWSER_HOME = 0xACu16
	BrowserHome          = 0xACu16,
	/// VK_VOLUME_MUTE = 0xADu16
	VolumeMute           = 0xADu16,
	/// VK_VOLUME_DOWN = 0xAEu16
	VolumeDown           = 0xAEu16,
	/// VK_VOLUME_UP = 0xAFu16
	VolumeUp             = 0xAFu16,
	/// VK_MEDIA_NEXT_TRACK = 0xB0u16
	MediaNextTrack       = 0xB0u16,
	/// VK_MEDIA_PREV_TRACK = 0xB1u16
	MediaPrevTrack       = 0xB1u16,
	/// VK_MEDIA_STOP = 0xB2u16
	MediaStop            = 0xB2u16,
	/// VK_MEDIA_PLAY_PAUSE = 0xB3u16
	MediaPlayPause       = 0xB3u16,
	/// VK_LAUNCH_MAIL = 0xB4u16
	LaunchMail           = 0xB4u16,
	/// VK_LAUNCH_MEDIA_SELECT = 0xB5u16
	LaunchMediaSelect    = 0xB5u16,
	/// VK_LAUNCH_APP1 = 0xB6u16
	LaunchApp1           = 0xB6u16,
	/// VK_LAUNCH_APP2 = 0xB7u16
	LaunchApp2           = 0xB7u16,
	/// VK_OEM_1 = 0xBAu16
	Oem1                 = 0xBAu16,
	/// VK_OEM_PLUS = 0xBBu16
	OemPlus              = 0xBBu16,
	/// VK_OEM_COMMA = 0xBCu16
	OemComma             = 0xBCu16,
	/// VK_OEM_MINUS = 0xBDu16
	OemMinus             = 0xBDu16,
	/// VK_OEM_PERIOD = 0xBEu16
	OemPeriod            = 0xBEu16,
	/// VK_OEM_2 = 0xBFu16
	Oem2                 = 0xBFu16,
	/// VK_OEM_3 = 0xC0u16
	Oem3                 = 0xC0u16,
	/// VK_GAMEPAD_A = 0xC3u16
	GamePadA             = 0xC3u16,
	/// VK_GAMEPAD_B = 0xC4u16
	GamePadB             = 0xC4u16,
	/// VK_GAMEPAD_X = 0xC5u16
	GamePadX             = 0xC5u16,
	/// VK_GAMEPAD_Y = 0xC6u16
	GamePadY             = 0xC6u16,
	/// VK_GAMEPAD_RIGHT_SHOULDER = 0xC7u16
	GamePadRightShoulder = 0xC7u16,
	/// VK_GAMEPAD_LEFT_SHOULDER = 0xC8u16
	GamePadLeftShoulder  = 0xC8u16,
	/// VK_GAMEPAD_LEFT_TRIGGER = 0xC9u16
	GamePadLeftTrigger   = 0xC9u16,
	/// VK_GAMEPAD_RIGHT_TRIGGER = 0xCAu16
	GamePadRightTrigger  = 0xCAu16,
	/// VK_GAMEPAD_DPAD_UP = 0xCBu16
	GamePadDpadUp        = 0xCBu16,
	/// VK_GAMEPAD_DPAD_DOWN = 0xCCu16
	GamePadDpadDown      = 0xCCu16,
	/// VK_GAMEPAD_DPAD_LEFT = 0xCDu16
	GamePadDpadLeft      = 0xCDu16,
	/// VK_GAMEPAD_DPAD_RIGHT = 0xCEu16
	GamePadDpadRight     = 0xCEu16,
	/// VK_GAMEPAD_MENU = 0xCFu16
	GamePadMenu          = 0xCFu16,
	/// VK_GAMEPAD_VIEW = 0xD0u16
	GamePadView          = 0xD0u16,
	/// VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON = 0xD1u16
	GamePadLeftThumbStickButton = 0xD1u16,
	/// VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON = 0xD2u16
	GamePadRightThumbStickButton = 0xD2u16,
	/// VK_GAMEPAD_LEFT_THUMBSTICK_UP = 0xD3u16
	GamePadLeftThumbStickUp = 0xD3u16,
	/// VK_GAMEPAD_LEFT_THUMBSTICK_DOWN = 0xD4u16
	GamePadLeftThumbStickDown = 0xD4u16,
	/// VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT = 0xD5u16
	GamePadLeftThumbStickRight = 0xD5u16,
	/// VK_GAMEPAD_LEFT_THUMBSTICK_LEFT = 0xD6u16
	GamePadLeftThumbStickLeft = 0xD6u16,
	/// VK_GAMEPAD_RIGHT_THUMBSTICK_UP = 0xD7u16
	GamePadRightThumbStickUp = 0xD7u16,
	/// VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN = 0xD8u16
	GamePadRightThumbStickDown = 0xD8u16,
	/// VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT = 0xD9u16
	GamePadRightThumbStickRight = 0xD9u16,
	/// VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT = 0xDAu16
	GamePadRightThumbStickLeft = 0xDAu16,
	/// VK_OEM_4 = 0xDBu16
	Oem4                 = 0xDBu16,
	/// VK_OEM_5 = 0xDCu16
	Oem5                 = 0xDCu16,
	/// VK_OEM_6 = 0xDDu16
	Oem6                 = 0xDDu16,
	/// VK_OEM_7 = 0xDEu16
	Oem7                 = 0xDEu16,
	/// VK_OEM_8 = 0xDFu16
	Oem8                 = 0xDFu16,
	/// VK_OEM_AX = 0xE1u16
	OemAx                = 0xE1u16,
	/// VK_OEM_102 = 0xE2u16
	Oem102               = 0xE2u16,
	/// VK_ICO_HELP = 0xE3u16
	IcoHelp              = 0xE3u16,
	/// VK_ICO_00 = 0xE4u16
	Ico00                = 0xE4u16,
	/// VK_PROCESSKEY = 0xE5u16
	ProcessKey           = 0xE5u16,
	/// VK_ICO_CLEAR = 0xE6u16
	IcoClear             = 0xE6u16,
	/// VK_PACKET = 0xE7u16
	Packet               = 0xE7u16,
	/// VK_OEM_RESET = 0xE9u16
	OemReset             = 0xE9u16,
	/// VK_OEM_JUMP = 0xEAu16
	OemJump              = 0xEAu16,
	/// VK_OEM_PA1 = 0xEBu16
	OemPa1               = 0xEBu16,
	/// VK_OEM_PA2 = 0xECu16
	OemPa2               = 0xECu16,
	/// VK_OEM_PA3 = 0xEDu16
	OemPa3               = 0xEDu16,
	/// VK_OEM_WSCTRL = 0xEEu16
	OemWSCtrl            = 0xEEu16,
	/// VK_OEM_CUSEL = 0xEFu16
	OemCUSel             = 0xEFu16,
	/// VK_OEM_ATTN = 0xF0u16
	OemAttn              = 0xF0u16,
	/// VK_OEM_FINISH = 0xF1u16
	OemFinish            = 0xF1u16,
	/// VK_OEM_COPY = 0xF2u16
	OemCopy              = 0xF2u16,
	/// VK_OEM_AUTO = 0xF3u16
	OemAuto              = 0xF3u16,
	/// VK_OEM_ENLW = 0xF4u16
	OemEnlW              = 0xF4u16,
	/// VK_OEM_BACKTAB = 0xF5u16
	OemBackTab           = 0xF5u16,
	/// VK_ATTN = 0xF6u16
	Attn                 = 0xF6u16,
	/// VK_CRSEL = 0xF7u16
	CRSel                = 0xF7u16,
	/// VK_EXSEL = 0xF8u16
	ExSel                = 0xF8u16,
	/// VK_EREOF = 0xF9u16
	EREof                = 0xF9u16,
	/// VK_PLAY = 0xFAu16
	Play                 = 0xFAu16,
	/// VK_ZOOM = 0xFBu16
	Zoom                 = 0xFBu16,
	/// VK_NONAME = 0xFCu16
	NoName               = 0xFCu16,
	/// VK_PA1 = 0xFDu16
	Pa1                  = 0xFDu16,
	/// VK_OEM_CLEAR = 0xFEu16
	OemClear             = 0xFEu16,
}

impl VirtualKey {
	/// VK_HANGEUL = 0x15u16
	pub const Hangeul             : Self = unsafe { transmute(0x15u16) };
	/// VK_HANGUL = 0x15u16
	pub const Hangul              : Self = unsafe { transmute(0x15u16) };
	/// VK_KANJI = 0x19u16
	pub const Kanji               : Self = unsafe { transmute(0x19u16) };
	/// VK_OEM_FJ_JISHO = 0x92u16
	pub const OemFjJisho          : Self = unsafe { transmute(0x92u16) };
}

