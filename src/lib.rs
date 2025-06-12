// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! # MCP Protocol Client
//!
//! Client library for the Model Context Protocol (MCP).
//!
//! This crate provides a high-level, ergonomic API for building MCP clients.
//! It handles connection management, capability negotiation, and transport
//! abstraction.

pub mod builder;
pub mod client;
pub mod error;
pub mod transport;

pub use builder::ClientBuilder;
pub use client::Client;
pub use error::{ClientError, ClientResult};

/// Re-export commonly used types
pub use mcp_protocol_types::*;
