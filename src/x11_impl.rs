use crate::Error;
use x11::{
    xss::{XScreenSaverAllocInfo, XScreenSaverQueryInfo},
    xlib::{XOpenDisplay, XDefaultScreen, XRootWindow, XFree, XCloseDisplay}
};
use std::os::raw::c_char;
use std::time::Duration;

// Mostly taken from https://stackoverflow.com/questions/222606/detecting-keyboard-mouse-activity-in-linux
pub fn get_idle_time() -> Result<Duration, Error> {

    unsafe {

        let info = XScreenSaverAllocInfo();
        let display = XOpenDisplay(0 as *const c_char);
        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);
        let status = XScreenSaverQueryInfo(display, root_window, info);
        let time = (*info).idle;

        XFree(info as *mut std::ffi::c_void);
        XCloseDisplay(display);

        if status == 1 {
            Ok(Duration::from_millis(time.into()))
        } else {
            Err(Error::new("Status not OK"))
        }

    }

}
