//! HEOS CLI Protocol client for Denon/Marantz devices
//!
//! This module provides a Rust implementation of the HEOS CLI protocol
//! for controlling HEOS-enabled devices over telnet (port 1255).

pub mod client;
pub mod commands;
pub mod types;

pub use client::HeosClient;
pub use types::*;
