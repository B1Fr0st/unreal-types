use std::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};

use crate::FVector;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FRotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
impl FRotator {
    pub fn to_fvector(self) -> FVector {
        let fyaw = self.yaw * PI / 180.0;
        let fpitch = self.pitch * PI / 180.0;
        let cospitch = fpitch.cos();

        FVector {
            x: fyaw.cos() * cospitch,
            y: fyaw.sin() * cospitch,
            z: fpitch.sin(),
        }
    }
}

impl Sub<Self> for FRotator {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FRotator {
            pitch: self.pitch - other.pitch,
            yaw: self.yaw - other.yaw,
            roll: self.roll - other.roll,
        }
    }
}

impl Mul<f32> for FRotator {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        FRotator {
            pitch: self.pitch * scalar,
            yaw: self.yaw * scalar,
            roll: self.roll * scalar,
        }
    }
}

impl Add<Self> for FRotator {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FRotator {
            pitch: self.pitch + other.pitch,
            yaw: self.yaw + other.yaw,
            roll: self.roll + other.roll,
        }
    }
}

impl FRotator {
    pub fn to_matrix(&self) -> [[f32; 4]; 4] {
        // Skidded.
        let origin = FRotator {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        };

        let rad_pitch = self.pitch * std::f32::consts::PI / 180.0;
        let rad_yaw = self.yaw * std::f32::consts::PI / 180.0;
        let rad_roll = self.roll * std::f32::consts::PI / 180.0;

        let sp = rad_pitch.sin();
        let cp = rad_pitch.cos();
        let sy = rad_yaw.sin();
        let cy = rad_yaw.cos();
        let sr = rad_roll.sin();
        let cr = rad_roll.cos();

        [
            [cp * cy, cp * sy, sp, 0.0],
            [
                sr * sp * cy - cr * sy,
                sr * sp * sy + cr * cy,
                -sr * cp,
                0.0,
            ],
            [
                -(cr * sp * cy + sr * sy),
                cy * sr - cr * sp * sy,
                cr * cp,
                0.0,
            ],
            [origin.roll, origin.yaw, origin.pitch, 1.0],
        ]
    }
}
