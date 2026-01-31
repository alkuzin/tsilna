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
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn calculate_checksum(buffer: &[u8]) -> u16 {
    let len = buffer.len();

    if len == 0 {
        return 0xFFFF;
    }

    let mut sum: u32 = 0;

    let mut chunks = buffer.chunks_exact(2);
    for chunk in &mut chunks {
        if let &[b0, b1] = chunk {
            let word = u16::from_be_bytes([b0, b1]);
            sum += u32::from(word);
        }
    }

    if let Some(&last_byte) = chunks.remainder().first() {
        sum += u32::from(last_byte) << 8;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        assert_eq!(calculate_checksum(&[]), 0xFFFF);
    }

    #[test]
    fn test_even_length_buffer() {
        let data = [0x00, 0x01, 0xf2, 0x03];
        assert_eq!(calculate_checksum(&data), 0x0DFB);
    }

    #[test]
    fn test_odd_length_buffer() {
        let data = [0x01, 0x02, 0x03];
        assert_eq!(calculate_checksum(&data), 0xFBFD);
    }

    #[test]
    fn test_carry_handling() {
        let data = [0xFF, 0xFF, 0x00, 0x01];
        assert_eq!(calculate_checksum(&data), 0xFFFE);
    }

    #[test]
    fn test_rfc_example() {
        let data = [0x01, 0x00, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
        assert_eq!(calculate_checksum(&data), 0x210E);
    }
}
