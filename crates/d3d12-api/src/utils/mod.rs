pub mod state_object_desc;
pub mod raytracing;
pub mod hlsl;
pub mod simd_math;

pub use state_object_desc::*;

use std::ops::{Add, BitAnd, Not, Sub};

pub fn align<T, U>(value: T, alignment: U) -> T
    where T: Copy + From<u8> + From<U> + Sub<Output=T> + Add<Output=T> + BitAnd<Output=T> + Not<Output=T>,
          U: Copy
{
    (value + T::from(alignment) - T::from(1)) & !(T::from(alignment) - T::from(1))
}

