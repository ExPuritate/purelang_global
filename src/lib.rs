#![feature(marker_trait_attr)]
#![feature(macro_metavar_expr)]
#![feature(can_vector)]
#![feature(write_all_vectored)]
#![feature(read_buf)]
#![feature(core_io_borrowed_buf)]
#![feature(pattern)]
#![feature(decl_macro)]
#![feature(panic_internals)]
#![feature(format_args_nl)]
#![feature(iterator_try_collect)]
#![allow(internal_features)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("unsupported");

extern crate proc_macros;

#[doc(hidden)]
pub extern crate iota;
#[doc(hidden)]
pub extern crate paste;

pub mod attrs;
pub mod configs;
pub mod errors;
pub mod find_util;
pub mod instruction;
pub mod io_utils;
pub mod macros;
pub mod traits;

pub mod color;
pub mod path_searcher;
mod string_name;
mod string_reference;

pub use macros::*;
pub use string_name::StringName;
pub use string_reference::{StringMethodReference, StringTypeReference};

// Re-exports
pub use anyhow::{Error, Result};
pub use borsh;
pub use cfg_if::cfg_if;
pub use derive_ctor;
pub use faststr::FastStr;
pub use getset;
pub use indexmap::{IndexMap, IndexSet, indexmap};
pub use num_enum;
pub use proc_macros::*;
