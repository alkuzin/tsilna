// SPDX-License-Identifier: Apache-2.0.
// Copyright (C) 2026-present tsilna-nav project and contributors.

//! Declarations for orientation of the body in 3D space.

use crate::{na::{self, RealField}, Quaternion};

/// Alias for 32-bit version of Euler angles.
pub type Euler32 = Euler<f32>;

/// Alias for 64-bit version of Euler angles.
pub type Euler64 = Euler<f64>;

/// Represents the orientation using the Euler angle convention.
pub struct Euler<T: RealField + Copy> {
    /// Rotation around X-axis in **radians**.
    pub roll: T,
    /// Rotation around Y-axis in **radians**.
    pub pitch: T,
    /// Rotation around Z-axis in **radians**.
    pub yaw: T,
}

impl<T: RealField + Copy> Euler<T> {
    /// Construct new `Euler` object from radians.
    ///
    /// # Parameters
    /// - `roll` - given rotation around X-axis in **radians**.
    /// - `pitch` - given rotation around Y-axis in **radians**.
    /// - `yaw` - given rotation around Z-axis in **radians**.
    ///
    /// # Returns
    /// - New `Euler` object.
    #[must_use]
    pub const fn from_radians(roll: T, pitch: T, yaw: T) -> Self {
        Self { roll, pitch, yaw }
    }

    /// Construct new `Euler` object from degrees.
    ///
    /// # Parameters
    /// - `roll` - given rotation around X-axis in **degrees**.
    /// - `pitch` - given rotation around Y-axis in **degrees**.
    /// - `yaw` - given rotation around Z-axis in **degrees**.
    ///
    /// # Returns
    /// - New `Euler` object.
    #[must_use]
    pub fn from_degrees(roll: T, pitch: T, yaw: T) -> Self {
        let to_rad = T::pi() / na::convert(180.0);

        Self {
            roll: roll * to_rad,
            pitch: pitch * to_rad,
            yaw: yaw * to_rad,
        }
    }

    /// Construct new `Euler` object from given quaternion.
    ///
    /// # Parameters
    /// - `q` - given quaternion to construct from.
    ///
    /// # Returns
    /// - New `Euler` object.
    #[inline]
    #[must_use]
    pub fn from_quaternion(q: Quaternion<T>) -> Self {
        let (roll, pitch, yaw) = q.euler_angles();
        Self { roll, pitch, yaw }
    }

    /// Convert the attitude to quaternion.
    ///
    /// # Returns
    /// - New unit quaternion from Euler angles.
    #[inline]
    #[must_use]
    pub fn to_quaternion(&self) -> Quaternion<T> {
        Quaternion::<T>::from_euler_angles(self.roll, self.pitch, self.yaw)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use core::f64::consts;
    use super::*;

    #[test]
    fn test_euler_from_radians() {
        let e = Euler32::from_radians(1.0, 0.5, -1.0);
        assert_eq!(e.roll, 1.0);
        assert_eq!(e.pitch, 0.5);
        assert_eq!(e.yaw, -1.0);
    }

    #[test]
    fn test_euler_from_degrees() {
        // 180 degrees should be PI radians.
        let e = Euler64::from_degrees(180.0, 90.0, 45.0);

        assert_relative_eq!(e.roll, consts::PI);
        assert_relative_eq!(e.pitch, consts::FRAC_PI_2);
        assert_relative_eq!(e.yaw, consts::FRAC_PI_4);
    }

    #[test]
    fn test_quaternion_round_trip() {
        let original = Euler32::from_degrees(10.0, 20.0, 30.0);
        let quat = original.to_quaternion();
        let result = Euler32::from_quaternion(quat);

        // Checking if values returned are the same within float precision.
        assert_relative_eq!(original.roll, result.roll, epsilon = 1e-6);
        assert_relative_eq!(original.pitch, result.pitch, epsilon = 1e-6);
        assert_relative_eq!(original.yaw, result.yaw, epsilon = 1e-6);
    }

    #[test]
    fn test_gimbal_lock_safety() {
        let gimbal_lock = Euler32::from_degrees(0.0, 90.0, 0.0);
        let quat = gimbal_lock.to_quaternion();

        assert_relative_eq!(quat.norm(), 1.0);
    }
}
