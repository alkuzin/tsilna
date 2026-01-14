// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! `tsilna-nav` - memory-safe autonomous navigation framework
//! suitable both for `embedded systems` & general-purpose operating systems.

#![no_std]

#[cfg(feature = "protocol")]
pub use tsilna_protocol as protocol;
