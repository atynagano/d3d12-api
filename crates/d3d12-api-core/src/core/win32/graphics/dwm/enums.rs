#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// DWMWINDOWATTRIBUTE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmWindowAttribute
{
	/// DWMWA_NCRENDERING_ENABLED = 0x1i32
	NcRenderingEnabled   = 0x1i32,
	/// DWMWA_NCRENDERING_POLICY = 0x2i32
	NcRenderingPolicy    = 0x2i32,
	/// DWMWA_TRANSITIONS_FORCEDISABLED = 0x3i32
	TransitionsForceDisabled = 0x3i32,
	/// DWMWA_ALLOW_NCPAINT = 0x4i32
	AllowNcPaint         = 0x4i32,
	/// DWMWA_CAPTION_BUTTON_BOUNDS = 0x5i32
	CaptionButtonBounds  = 0x5i32,
	/// DWMWA_NONCLIENT_RTL_LAYOUT = 0x6i32
	NonClientRtlLayout   = 0x6i32,
	/// DWMWA_FORCE_ICONIC_REPRESENTATION = 0x7i32
	ForceIconicRepresentation = 0x7i32,
	/// DWMWA_FLIP3D_POLICY = 0x8i32
	Flip3dPolicy         = 0x8i32,
	/// DWMWA_EXTENDED_FRAME_BOUNDS = 0x9i32
	ExtendedFrameBounds  = 0x9i32,
	/// DWMWA_HAS_ICONIC_BITMAP = 0xAi32
	HasIconicBitmap      = 0xAi32,
	/// DWMWA_DISALLOW_PEEK = 0xBi32
	DisallowPeek         = 0xBi32,
	/// DWMWA_EXCLUDED_FROM_PEEK = 0xCi32
	ExcludedFromPeek     = 0xCi32,
	/// DWMWA_CLOAK = 0xDi32
	Cloak                = 0xDi32,
	/// DWMWA_CLOAKED = 0xEi32
	Cloaked              = 0xEi32,
	/// DWMWA_FREEZE_REPRESENTATION = 0xFi32
	FreezeRepresentation = 0xFi32,
	/// DWMWA_PASSIVE_UPDATE_MODE = 0x10i32
	PassiveUpdateMode    = 0x10i32,
	/// DWMWA_USE_HOSTBACKDROPBRUSH = 0x11i32
	UseHostBackdropBrush = 0x11i32,
	/// DWMWA_USE_IMMERSIVE_DARK_MODE = 0x14i32
	UseImmersiveDarkMode = 0x14i32,
	/// DWMWA_WINDOW_CORNER_PREFERENCE = 0x21i32
	WindowCornerPreference = 0x21i32,
	/// DWMWA_BORDER_COLOR = 0x22i32
	BorderColor          = 0x22i32,
	/// DWMWA_CAPTION_COLOR = 0x23i32
	CaptionColor         = 0x23i32,
	/// DWMWA_TEXT_COLOR = 0x24i32
	TextColor            = 0x24i32,
	/// DWMWA_VISIBLE_FRAME_BORDER_THICKNESS = 0x25i32
	VisibleFrameBorderThickness = 0x25i32,
	/// DWMWA_LAST = 0x26i32
	Last                 = 0x26i32,
}

/// DWM_WINDOW_CORNER_PREFERENCE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmWindowCornerPreference
{
	/// DWMWCP_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// DWMWCP_DONOTROUND = 0x1i32
	DoNotRound           = 0x1i32,
	/// DWMWCP_ROUND = 0x2i32
	Round                = 0x2i32,
	/// DWMWCP_ROUNDSMALL = 0x3i32
	RoundSmall           = 0x3i32,
}

/// DWMNCRENDERINGPOLICY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmNcRenderingPolicy
{
	/// DWMNCRP_USEWINDOWSTYLE = 0x0i32
	UseWindowStyle       = 0x0i32,
	/// DWMNCRP_DISABLED = 0x1i32
	Disabled             = 0x1i32,
	/// DWMNCRP_ENABLED = 0x2i32
	Enabled              = 0x2i32,
	/// DWMNCRP_LAST = 0x3i32
	Last                 = 0x3i32,
}

/// DWMFLIP3DWINDOWPOLICY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmFlip3dWindowPolicy
{
	/// DWMFLIP3D_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// DWMFLIP3D_EXCLUDEBELOW = 0x1i32
	ExcludeBelow         = 0x1i32,
	/// DWMFLIP3D_EXCLUDEABOVE = 0x2i32
	ExcludeAbove         = 0x2i32,
	/// DWMFLIP3D_LAST = 0x3i32
	Last                 = 0x3i32,
}

/// DWM_SOURCE_FRAME_SAMPLING
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmSourceFrameSampling
{
	/// DWM_SOURCE_FRAME_SAMPLING_POINT = 0x0i32
	Point                = 0x0i32,
	/// DWM_SOURCE_FRAME_SAMPLING_COVERAGE = 0x1i32
	Coverage             = 0x1i32,
	/// DWM_SOURCE_FRAME_SAMPLING_LAST = 0x2i32
	Last                 = 0x2i32,
}

/// DWMTRANSITION_OWNEDWINDOW_TARGET
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwmTransitionOwnedWindowTarget
{
	/// DWMTRANSITION_OWNEDWINDOW_NULL = -0x1i32
	Null                 = -0x1i32,
	/// DWMTRANSITION_OWNEDWINDOW_REPOSITION = 0x0i32
	Reposition           = 0x0i32,
}

/// GESTURE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GestureType
{
	/// GT_PEN_TAP = 0x0i32
	PenTap               = 0x0i32,
	/// GT_PEN_DOUBLETAP = 0x1i32
	PenDoubleTap         = 0x1i32,
	/// GT_PEN_RIGHTTAP = 0x2i32
	PenRightTap          = 0x2i32,
	/// GT_PEN_PRESSANDHOLD = 0x3i32
	PenPresSandHold      = 0x3i32,
	/// GT_PEN_PRESSANDHOLDABORT = 0x4i32
	PenPresSandHoldAbort = 0x4i32,
	/// GT_TOUCH_TAP = 0x5i32
	TouchTap             = 0x5i32,
	/// GT_TOUCH_DOUBLETAP = 0x6i32
	TouchDoubleTap       = 0x6i32,
	/// GT_TOUCH_RIGHTTAP = 0x7i32
	TouchRightTap        = 0x7i32,
	/// GT_TOUCH_PRESSANDHOLD = 0x8i32
	TouchPresSandHold    = 0x8i32,
	/// GT_TOUCH_PRESSANDHOLDABORT = 0x9i32
	TouchPresSandHoldAbort = 0x9i32,
	/// GT_TOUCH_PRESSANDTAP = 0xAi32
	TouchPresSandTap     = 0xAi32,
}

bitflags::bitflags! {
	/// DWM_SHOWCONTACT
	#[repr(transparent)]
	pub struct DwmShowContact: u32 {
		/// DWMSC_DOWN = 0x1u32
		const Down                 = 0x1u32;
		/// DWMSC_UP = 0x2u32
		const Up                   = 0x2u32;
		/// DWMSC_DRAG = 0x4u32
		const Drag                 = 0x4u32;
		/// DWMSC_HOLD = 0x8u32
		const Hold                 = 0x8u32;
		/// DWMSC_PENBARREL = 0x10u32
		const PenBarrel            = 0x10u32;
		/// DWMSC_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DWMSC_ALL = 0xFFFFFFFFu32
		const All                  = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// DWM_TAB_WINDOW_REQUIREMENTS
	#[repr(transparent)]
	pub struct DwmTabWindowRequirements: u32 {
		/// DWMTWR_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DWMTWR_IMPLEMENTED_BY_SYSTEM = 0x1u32
		const ImplementedBySystem  = 0x1u32;
		/// DWMTWR_WINDOW_RELATIONSHIP = 0x2u32
		const WindowRelationship   = 0x2u32;
		/// DWMTWR_WINDOW_STYLES = 0x4u32
		const WindowStyles         = 0x4u32;
		/// DWMTWR_WINDOW_REGION = 0x8u32
		const WindowRegion         = 0x8u32;
		/// DWMTWR_WINDOW_DWM_ATTRIBUTES = 0x10u32
		const WindowDwmAttributes  = 0x10u32;
		/// DWMTWR_WINDOW_MARGINS = 0x20u32
		const WindowMargins        = 0x20u32;
		/// DWMTWR_TABBING_ENABLED = 0x40u32
		const TabbingEnabled       = 0x40u32;
		/// DWMTWR_USER_POLICY = 0x80u32
		const UserPolicy           = 0x80u32;
		/// DWMTWR_GROUP_POLICY = 0x100u32
		const GroupPolicy          = 0x100u32;
		/// DWMTWR_APP_COMPAT = 0x200u32
		const AppComPat            = 0x200u32;
	}
}

