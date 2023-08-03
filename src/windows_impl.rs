use crate::error::Error;
use std::time::Duration;
use winapi::um::{
    sysinfoapi::GetTickCount,
    winuser::{GetLastInputInfo, LASTINPUTINFO, PLASTINPUTINFO},
};

// Based on https://bitbucket.org/pidgin/main/src/8066acc5ed9306c5a53da8f66f50fb5cf38782c7/pidgin/win32/gtkwin32dep.c#lines-597

pub fn get_idle_time() -> Result<Duration, Error> {
    let now = unsafe { GetTickCount() };

    let mut last_input_info = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };

    let p_last_input_info: PLASTINPUTINFO = &mut last_input_info as *mut LASTINPUTINFO;

    let ok = unsafe { GetLastInputInfo(p_last_input_info) } != 0;

    match ok {
        true => {
            let millis = now - last_input_info.dwTime;
            Ok(Duration::from_millis(millis as u64))
        }
        false => Err(Error::new("GetLastInputInfo failed")),
    }
}
