//! Autogenerated traits for OpenRPC specs.
//!
//! This crate interprets each non-hidden file in the `specs` folder as an
//! OpenRPC document,
//! and generates a module according to the filename.
//! Changes to those files will automatically be reflected in the code.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
