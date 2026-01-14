// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! Utilities for protocols.

/// Calculate Internet checksum (RFC 1071).
///
/// # Parameters
/// - `buffer` - given buffer to handle.
///
/// # Returns
/// - Internet checksum of a given buffer.
pub fn calculate_checksum(buffer: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let len = buffer.len();
    let mut i = 0;

    while i < len - 1 {
        let word = u16::from_ne_bytes([buffer[i], buffer[i + 1]]);
        sum += word as u32;
        i += 2;
    }

    if i < len {
        sum += buffer[i] as u32;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}
