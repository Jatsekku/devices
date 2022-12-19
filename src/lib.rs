#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![doc = include_str!("../README.md")]

mod error;
mod info;
mod path;

use cfg_if::cfg_if;
pub use error::Error;
pub use info::DeviceInfo;
pub use path::DevicePath;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod win32;

/// Information about system devices.
pub struct Devices;

impl Devices {
    /// Retrieve a list of all connected devices.
    /// # Errors
    /// If the platform is unsupported or there is an issue retrieving the list of devices, an error is returned.
    pub fn get() -> Result<Vec<DeviceInfo>, Error> {
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                linux::get_devices()
            } else {
                Err(Error::UnsupportedPlatform)
            }
        }
    }
}
