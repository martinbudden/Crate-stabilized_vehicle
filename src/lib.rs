#![cfg_attr(feature = "simd", feature(portable_simd))]
#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(unused_results)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]

mod ahrs;
mod imu_filters;
mod vehicle_controller;

pub use vehicle_controller::{VehicleControl, VehicleController};

pub use ahrs::{Ahrs, AhrsData};

pub use imu_filters::{FilterAccGyro, ImuFilterBank, ImuFilterBankConfig};
