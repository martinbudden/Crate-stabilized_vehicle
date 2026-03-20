#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod ahrs;
mod imu_filters;
mod vehicle_controller;

pub use vehicle_controller::{VehicleController, VehicleControllerState};

pub use ahrs::{Ahrs, AhrsData};

pub use imu_filters::{ImuFilters, ImuFiltersConfig, ImuFiltersState};
