// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! This crate consist of binary protocols implementation and utilities
//! to ensure the integrity of transmitted data.
//!
//! Supported binary protocols:
//! - `IDTP (Inertial Measurement Unit Data Transfer Protocol)` — it is a binary
//!   protocol that can be used by different transport layers, such as SPI, I2C,
//!   UART, UDP or TCP.
//!   This protocol designed for transferring navigation data in systems with
//!   strict real-time requirements (unmanned vehicles, robotics).
//!   IDTP solves the problem of unifying data exchange between different types
//!   of inertial measurement units (IMU) and host systems, providing
//!   a multi-level data integrity checking.
//!
//! Supported utilities:
//!
//! - `checksum (RFC 1071)`
//! - `crc`

#![no_std]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::todo,
    clippy::unreachable,
    missing_docs
)]

pub mod utils;
pub use idtp;
pub use crc;
