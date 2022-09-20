#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]


mod _mod;
pub use self::_mod::*;
mod structs;
pub use self::structs::*;
mod enums;
pub use self::enums::*;
mod apis;
pub use self::apis::*;
mod constants;
pub use self::constants::*;

pub mod direct3d12;
pub mod direct2d;
pub mod dxgi;
pub mod gdi;
pub mod direct3d;
pub mod dwm;
pub mod direct_write;
pub mod imaging;
