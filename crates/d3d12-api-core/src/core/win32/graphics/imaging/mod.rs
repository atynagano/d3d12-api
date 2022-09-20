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

mod bitmap_source;
pub use self::bitmap_source::*;
mod bitmap;
pub use self::bitmap::*;
mod color_context;
pub use self::color_context::*;
mod imaging_factory;
pub use self::imaging_factory::*;
mod palette;
pub use self::palette::*;
mod bitmap_lock;
pub use self::bitmap_lock::*;
mod bitmap_decoder;
pub use self::bitmap_decoder::*;
mod component_info;
pub use self::component_info::*;
mod bitmap_encoder;
pub use self::bitmap_encoder::*;
mod format_converter;
pub use self::format_converter::*;
mod bitmap_scaler;
pub use self::bitmap_scaler::*;
mod bitmap_clipper;
pub use self::bitmap_clipper::*;
mod bitmap_flip_rotator;
pub use self::bitmap_flip_rotator::*;
mod stream;
pub use self::stream::*;
mod color_transform;
pub use self::color_transform::*;
mod fast_metadata_encoder;
pub use self::fast_metadata_encoder::*;
mod bitmap_frame_decode;
pub use self::bitmap_frame_decode::*;
mod metadata_query_writer;
pub use self::metadata_query_writer::*;
mod metadata_query_reader;
pub use self::metadata_query_reader::*;
mod bitmap_decoder_info;
pub use self::bitmap_decoder_info::*;
mod bitmap_codec_info;
pub use self::bitmap_codec_info::*;
mod bitmap_encoder_info;
pub use self::bitmap_encoder_info::*;
mod bitmap_frame_encode;
pub use self::bitmap_frame_encode::*;
