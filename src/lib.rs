/*!
Get the idle time of a user

The time returned is the time since the last user input event

See the [`README.md`](https://github.com/olback/user-idle-rs/blob/master/README.md) for more information

Example:
```
use user_idle::UserIdle;
let idle = UserIdle::get_time().unwrap();
let idle_seconds = idle.as_seconds();
let idle_minutes = idle.as_minutes();
```
*/

mod error;

use std::time::Duration;

pub use error::Error;

#[cfg(all(target_os = "linux", not(feature = "dbus")))]
#[path = "x11_impl.rs"]
mod idle;

#[cfg(all(target_os = "linux", feature = "dbus"))]
#[path = "dbus_impl.rs"]
mod idle;

#[cfg(target_os = "windows")]
#[path = "windows_impl.rs"]
mod idle;

#[cfg(target_os = "macos")]
#[path = "macos_impl.rs"]
mod idle;

pub struct UserIdle {
    duration: Duration,
}

impl UserIdle {
    /// Get the idle time
    pub fn get_time() -> Result<Self, Error> {
        Ok(UserIdle {
            duration: idle::get_idle_time()?,
        })
    }

    /**
    Get time in milliseconds

    Note: Only MacOS provides this level of resolution,
    other Operating Systems will provide the same value as
    `self.as_milliseconds() * 1_000_000`
    */
    pub fn as_nanoseconds(&self) -> u128 {
        self.duration.as_nanos()
    }

    /**
    Get time in milliseconds

    Note: Not all of the dbus screen savers provided
    this level of resolution, in those cases this will
    provide the same value as `self.as_seconds() * 1000`
    */
    pub fn as_milliseconds(&self) -> u128 {
        self.duration.as_millis()
    }

    /// Get time in seconds
    pub fn as_seconds(&self) -> u64 {
        self.duration.as_secs()
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

    /// Convert to a [Duration]
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use super::UserIdle;

    const TEST_SECS: u64 = 10;

    #[test]
    fn main() {
        sleep(Duration::from_secs(TEST_SECS));

        let idle = UserIdle::get_time().unwrap();
        assert_eq!(idle.as_seconds(), TEST_SECS);
    }
}
