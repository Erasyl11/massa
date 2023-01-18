//! Copyright (c) 2022 MASSA LABS <info@massa.net>
//! Json RPC API for a massa-node
#![feature(async_closure)]
#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]

/// gRPC API implementation
pub mod api;
/// gRPC configuration
pub mod config;
/// models error
pub mod error;
