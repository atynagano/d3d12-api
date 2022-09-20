use std::mem::{MaybeUninit, size_of, size_of_val, transmute};
use crate::core::win32::foundation::{Bool, HResult, HWnd, Rect};
use crate::core::win32::graphics::gdi::ColorRef;
use super::*;

pub trait DwmWindowAttributeProvider {
    const ATTRIBUTE: DwmWindowAttribute;
}

pub trait DwmSetWindowAttributeProvider: DwmWindowAttributeProvider {}

pub trait DwmGetWindowAttributeProvider: DwmWindowAttributeProvider {
    type Target;
}

pub struct DwmTransitionsForceDisabled(pub Bool);

pub struct DwmAllowNcPaint(pub Bool);

pub struct DwmNonClientRtlLayout(pub Bool);

pub struct DwmForceIconicRepresentation(pub Bool);

pub struct DwmHasIconicBitmap(pub Bool);

pub struct DwmDisallowPeek(pub Bool);

pub struct DwmExcludedFromPeek(pub Bool);

pub struct DwmUseHostBackdropBrush(pub Bool);

pub struct DwmUseImmersiveDarkMode(pub Bool);

pub struct DwmCloak(pub Bool);

pub struct DwmBorderColor(pub ColorRef);

pub struct DwmCaptionColor(pub ColorRef);

pub struct DwmTextColor(pub ColorRef);

impl DwmWindowAttributeProvider for DwmNcRenderingPolicy { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::NcRenderingPolicy; }

impl DwmSetWindowAttributeProvider for DwmNcRenderingPolicy {}

impl DwmWindowAttributeProvider for DwmTransitionsForceDisabled { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::TransitionsForceDisabled; }

impl DwmSetWindowAttributeProvider for DwmTransitionsForceDisabled {}

impl DwmWindowAttributeProvider for DwmAllowNcPaint { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::AllowNcPaint; }

impl DwmSetWindowAttributeProvider for DwmAllowNcPaint {}

impl DwmWindowAttributeProvider for DwmNonClientRtlLayout { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::NonClientRtlLayout; }

impl DwmSetWindowAttributeProvider for DwmNonClientRtlLayout {}

impl DwmWindowAttributeProvider for DwmForceIconicRepresentation { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::ForceIconicRepresentation; }

impl DwmSetWindowAttributeProvider for DwmForceIconicRepresentation {}

impl DwmWindowAttributeProvider for DwmFlip3dWindowPolicy { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::Flip3dPolicy; }

impl DwmSetWindowAttributeProvider for DwmFlip3dWindowPolicy {}

impl DwmWindowAttributeProvider for DwmHasIconicBitmap { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::HasIconicBitmap; }

impl DwmSetWindowAttributeProvider for DwmHasIconicBitmap {}

impl DwmWindowAttributeProvider for DwmDisallowPeek { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::DisallowPeek; }

impl DwmSetWindowAttributeProvider for DwmDisallowPeek {}

impl DwmWindowAttributeProvider for DwmExcludedFromPeek { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::ExcludedFromPeek; }

impl DwmSetWindowAttributeProvider for DwmExcludedFromPeek {}

impl DwmWindowAttributeProvider for DwmCloak { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::Cloak; }

impl DwmSetWindowAttributeProvider for DwmCloak {}

//impl WindowAttribute for DwmFreezeRepresentation{ const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::FreezeRepresentation;}
//impl DwmSetWindowAttributeProvider for DwmFreezeRepresentation {}
//impl WindowAttribute for DwmPassiveUpdateMode{ const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::PassiveUpdateMode;}
impl DwmWindowAttributeProvider for DwmUseHostBackdropBrush { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::UseHostBackdropBrush; }

impl DwmSetWindowAttributeProvider for DwmUseHostBackdropBrush {}

impl DwmWindowAttributeProvider for DwmUseImmersiveDarkMode { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::UseImmersiveDarkMode; }

impl DwmSetWindowAttributeProvider for DwmUseImmersiveDarkMode {}

impl DwmWindowAttributeProvider for DwmWindowCornerPreference { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::WindowCornerPreference; }

impl DwmSetWindowAttributeProvider for DwmWindowCornerPreference {}

impl DwmWindowAttributeProvider for DwmBorderColor { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::BorderColor; }

impl DwmSetWindowAttributeProvider for DwmBorderColor {}

impl DwmWindowAttributeProvider for DwmCaptionColor { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::CaptionColor; }

impl DwmSetWindowAttributeProvider for DwmCaptionColor {}

impl DwmWindowAttributeProvider for DwmTextColor { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::TextColor; }

impl DwmSetWindowAttributeProvider for DwmTextColor {}

pub fn DwmSetWindowAttribute<A: DwmSetWindowAttributeProvider>(hwnd: HWnd, attribute: &A) -> Result<(), HResult> {
    unsafe {
        #[link(name = "dwmapi")]
        extern "system" {
            fn DwmSetWindowAttribute(hwnd: HWnd, attribute: DwmWindowAttribute, pv_attribute: *const u8, len: u32) -> HResult;
        }
        let _ret_ = DwmSetWindowAttribute(hwnd, A::ATTRIBUTE, transmute(attribute), size_of_val(attribute) as u32);
        _ret_.ok()
    }
}

pub struct DwmNcRenderingEnabled;

pub struct DwmCaptionButtonBounds;

pub struct DwmExtendedFrameBounds;

pub struct DwmVisibleFrameBorderThickness;

impl DwmWindowAttributeProvider for DwmNcRenderingEnabled { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::NcRenderingEnabled; }

impl DwmGetWindowAttributeProvider for DwmNcRenderingEnabled { type Target = Bool; }

impl DwmWindowAttributeProvider for DwmCaptionButtonBounds { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::CaptionButtonBounds; }

impl DwmGetWindowAttributeProvider for DwmCaptionButtonBounds { type Target = Rect; }

impl DwmWindowAttributeProvider for DwmExtendedFrameBounds { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::ExtendedFrameBounds; }

impl DwmGetWindowAttributeProvider for DwmExtendedFrameBounds { type Target = Rect; }

impl DwmWindowAttributeProvider for DwmVisibleFrameBorderThickness { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::VisibleFrameBorderThickness; }

impl DwmGetWindowAttributeProvider for DwmVisibleFrameBorderThickness { type Target = u32; }

// todo
#[repr(u32)]
pub enum DwmCloaked {
    A = 0
}

impl DwmWindowAttributeProvider for DwmCloaked { const ATTRIBUTE: DwmWindowAttribute = DwmWindowAttribute::Cloaked; }

impl DwmGetWindowAttributeProvider for DwmCloaked { type Target = (); }

pub fn DwmGetWindowAttribute<A: DwmGetWindowAttributeProvider>(hwnd: HWnd) -> Result<A::Target, HResult> {
    unsafe {
        #[link(name = "dwmapi")]
        extern "system" {
            fn DwmGetWindowAttribute(hwnd: HWnd, attribute: DwmWindowAttribute, pv_attribute: *const u8, len: u32) -> HResult;
        }
        let mut out = MaybeUninit::<A::Target>::zeroed();
        let _ret_ = DwmGetWindowAttribute(hwnd, A::ATTRIBUTE, out.as_mut_ptr() as _, size_of::<A::Target>() as u32);
        match _ret_.is_ok() {
            true => Ok(out.assume_init()),
            false => Err(_ret_)
        }
    }
}