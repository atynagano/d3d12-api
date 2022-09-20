pub mod state_object_desc;
pub mod raytracing;
pub mod hlsl;
pub mod simd_math;

pub use state_object_desc::*;

use std::ops::{Add, BitAnd, Not, Sub};
use d3d12_api_core::core::win32::graphics::direct3d12::D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT;

/*
pub fn align<T, U>(value: T, alignment: U) -> T
    where T: Copy + From<u8> + From<U> + Sub<Output=T> + Add<Output=T> + BitAnd<Output=T> + Not<Output=T>,
          U: Copy
{
    (value + T::from(alignment) - T::from(1)) & !(T::from(alignment) - T::from(1))
}*/
//pub const
fn align<const ALIGNMENT: u32>(value: u64) -> u64 {
    // note: error[E0401]: can't use generic parameters from outer function
    // const C: u32 = ALIGNMENT;
    // const CONST_ASSERT: usize = C.is_power_of_two() as u8 - 1;
    // const A: bool = ALIGNMENT.is_power_of_two();
    // note: error: generic parameters may not be used in const operations
    // do_nothing::<{ ALIGNMENT.is_power_of_two() as u8 - 1 }>();
    (value + ALIGNMENT as u64 - 1) & !(ALIGNMENT as u64 - 1)
}

const fn do_nothing<const N: u8>() {}

pub fn align_to_shader_record(value: u64) -> u64 {
    align::<{ D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT }>(value)
}
