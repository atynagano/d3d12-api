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

mod object;
pub use self::object::*;
mod device_sub_object;
pub use self::device_sub_object::*;
mod resource;
pub use self::resource::*;
mod keyed_mutex;
pub use self::keyed_mutex::*;
mod surface;
pub use self::surface::*;
mod surface1;
pub use self::surface1::*;
mod adapter;
pub use self::adapter::*;
mod output;
pub use self::output::*;
mod swap_chain;
pub use self::swap_chain::*;
mod factory;
pub use self::factory::*;
mod device;
pub use self::device::*;
mod factory1;
pub use self::factory1::*;
mod adapter1;
pub use self::adapter1::*;
mod device1;
pub use self::device1::*;
mod display_control;
pub use self::display_control::*;
mod output_duplication;
pub use self::output_duplication::*;
mod surface2;
pub use self::surface2::*;
mod resource1;
pub use self::resource1::*;
mod device2;
pub use self::device2::*;
mod swap_chain1;
pub use self::swap_chain1::*;
mod factory2;
pub use self::factory2::*;
mod adapter2;
pub use self::adapter2::*;
mod output1;
pub use self::output1::*;
mod device3;
pub use self::device3::*;
mod swap_chain2;
pub use self::swap_chain2::*;
mod output2;
pub use self::output2::*;
mod factory3;
pub use self::factory3::*;
mod decode_swap_chain;
pub use self::decode_swap_chain::*;
mod factory_media;
pub use self::factory_media::*;
mod swap_chain_media;
pub use self::swap_chain_media::*;
mod output3;
pub use self::output3::*;
mod swap_chain3;
pub use self::swap_chain3::*;
mod output4;
pub use self::output4::*;
mod factory4;
pub use self::factory4::*;
mod adapter3;
pub use self::adapter3::*;
mod output5;
pub use self::output5::*;
mod swap_chain4;
pub use self::swap_chain4::*;
mod device4;
pub use self::device4::*;
mod factory5;
pub use self::factory5::*;
mod adapter4;
pub use self::adapter4::*;
mod output6;
pub use self::output6::*;
mod factory6;
pub use self::factory6::*;
mod factory7;
pub use self::factory7::*;
mod info_queue;
pub use self::info_queue::*;
mod debug;
pub use self::debug::*;
mod debug1;
pub use self::debug1::*;
mod graphics_analysis;
pub use self::graphics_analysis::*;
pub mod common;
