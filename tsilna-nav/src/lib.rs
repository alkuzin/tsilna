// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! `tsilna-nav` - memory-safe autonomous navigation framework
//! suitable both for `embedded systems` & general-purpose operating systems.

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

#[cfg(feature = "protocol")]
pub use tsilna_protocol as protocol;

#[cfg(feature = "math")]
pub use tsilna_math as math;
