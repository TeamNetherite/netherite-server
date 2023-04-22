#![feature(core_intrinsics)]
#![feature(async_fn_in_trait)]
#![feature(decl_macro)]

use std::ops::Range;

// 1.19.0 - 1.19.4
pub const PROTOCOL_VERSIONS: Range<i32> = 759..=762;
// 1.19.0
pub const PRIMARY_PROTOCOL_VERSION: i32 = 759;

pub mod server;
pub mod packet;
pub mod types;
pub mod error;
pub mod model;
