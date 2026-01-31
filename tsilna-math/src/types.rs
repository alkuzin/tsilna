// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! Types and types aliases for math library.

use crate::na;

/// Alias for generic unit quaternion.
pub type Quaternion<T> = na::UnitQuaternion<T>;

/// Alias for 32-bit unit quaternion.
pub type Quat32 = Quaternion<f32>;

/// Alias for 64-bit unit quaternion.
pub type Quat64 = Quaternion<f64>;
