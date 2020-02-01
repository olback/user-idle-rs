/// Get a users idle time.
/// ```rust
/// use user_idle::UserIdle;
/// let idle = UserIdle::get_time().unwrap();
/// let idle_seconds = idle.as_seconds();
/// let idle_minutes = idle.as_minutes();
/// // Check the documentation for more methods
/// ```

mod error;
pub use error::Error;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub struct UserIdle {
    seconds: u32
}

impl UserIdle {

    /// Get the idle time
    pub fn get_time() -> Result<Self, Error> {

        #[cfg(unix)]
        let seconds = unix::get_idle_time()?;

        #[cfg(windows)]
        let seconds = windows::get_idle_time()?;

        Ok(UserIdle {
            seconds: seconds
        })

    }

    /// Get time in seconds
    pub fn as_seconds(&self) -> u32 {
        self.seconds
    }

    /// Get time in minutes
    pub fn as_minutes(&self) -> u32 {
        self.as_seconds() / 60
    }

    /// Get time in hours
    pub fn as_hours(&self) -> u32 {
        self.as_minutes() / 60
    }

    /// Get time in days
    pub fn as_days(&self) -> u32 {
        self.as_hours() / 24
    }

    /// Get time in weeks
    pub fn as_weeks(&self) -> u32 {
        self.as_days() / 7
    }

    /// Get time in months
    pub fn as_months(&self) -> u32 {
        self.as_weeks() / 4
    }

    /// Get time in years
    pub fn as_years(&self) -> u32 {
        self.as_months() / 12
    }

}
