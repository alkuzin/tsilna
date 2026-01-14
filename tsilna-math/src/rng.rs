// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! Random number generators.

use core::ops::Range;

/// Pseudo-random numbers generator.
///
/// Based on the `Xorshift algorithm` (George Marsaglia) - a type of LFSR
/// (Linear Feedback Shift Register).
pub struct Xorshift {
    /// Initial state of the generator.
    state: u32,
}

impl Xorshift {
    /// Construct new `Rng` object.
    ///
    /// # Parameters
    /// - `seed` - given initial state of the generator.
    ///
    /// # Returns
    /// - New `Rng` object.
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    /// Generate next pseudo-random number.
    ///
    /// # Returns
    /// - Next pseudo-random number as u32.
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    /// Generate float pseudo-random number in range [min, max].
    ///
    /// # Parameters
    /// - `range` - given range of number to generate.
    ///
    /// # Returns
    /// - Next pseudo-random number as f32.
    pub fn next_f32(&mut self, range: Range<f32>) -> f32 {
        let (min, max) = (range.start, range.end);
        let r = self.next_u32() as f32 / u32::MAX as f32;
        min + r * (max - min)
    }
}
