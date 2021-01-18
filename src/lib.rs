//! Get a users idle time.
//!
//! The time returned is the time since the last user input event.
//!
//! See the [`README.md`](https://github.com/olback/user-idle-rs/blob/master/README.md) for more information.
//!
//! Example:
//! ```rust
//! use user_idle::UserIdle;
//! let idle = UserIdle::get_time().unwrap();
//! let idle_seconds = idle.as_seconds();
//! let idle_minutes = idle.as_minutes();
//! // Check the documentation for more methods
//! ```

use std::time::Duration;
mod error;
pub use error::Error;

#[cfg(all(target_os = "linux", not(feature = "dbus")))]
mod x11_impl;

#[cfg(all(target_os = "linux", feature = "dbus"))]
mod dbus_impl;

#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "macos")]
mod macos_impl;

pub struct UserIdle {
    seconds: u64
}

impl UserIdle {

    /// Get the idle time
    pub fn get_time() -> Result<Self, Error> {

        #[cfg(all(target_os = "linux", not(feature = "dbus")))]
        let seconds = x11_impl::get_idle_time()?;

        #[cfg(all(target_os = "linux", feature = "dbus"))]
        let seconds = dbus_impl::get_idle_time()?;

        #[cfg(target_os = "windows")]
        let seconds = windows_impl::get_idle_time()?;

        #[cfg(target_os = "macos")]
        let seconds = macos_impl::get_idle_time()?;

        Ok(UserIdle {
            seconds: seconds
        })

    }

    /// Get time in seconds
    pub fn as_seconds(&self) -> u64 {
        self.seconds
    }

    /// Get time in minutes
    pub fn as_minutes(&self) -> u64 {
        self.as_seconds() / 60
    }

    /// Get time in hours
    pub fn as_hours(&self) -> u64 {
        self.as_minutes() / 60
    }

    /// Get time in days
    pub fn as_days(&self) -> u64 {
        self.as_hours() / 24
    }

    /// Get time in weeks
    pub fn as_weeks(&self) -> u64 {
        self.as_days() / 7
    }

    /// Get time in months
    pub fn as_months(&self) -> u64 {
        self.as_weeks() / 4
    }

    /// Get time in years
    pub fn as_years(&self) -> u64 {
        self.as_months() / 12
    }

    /// Convert to a std::time::Duration
    pub fn duration(&self) -> Duration {
        Duration::from_secs(self.seconds)
    }

}

// #[cfg(test)]
// mod tests {

//     use super::UserIdle;

//     #[test]
//     fn main() {

//         std::thread::sleep(std::time::Duration::from_secs(10));

//         let idle = UserIdle::get_time().unwrap();

//         println!("Idle for: {} seconds", idle.as_seconds());

//     }

// }
