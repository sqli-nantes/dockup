#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[cfg(feature = "serde_macros")]
include!("program_definition.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/program_definition.rs"));

#[macro_use]
extern crate log;

mod system;
pub mod config;
mod files;
pub mod logger;
pub mod program;
