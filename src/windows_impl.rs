use crate::error::Error;
use winapi::um::{
    winuser::{
        LASTINPUTINFO,
        PLASTINPUTINFO,
        GetLastInputInfo
    },
    sysinfoapi::GetTickCount
};

// Based on https://bitbucket.org/pidgin/main/src/8066acc5ed9306c5a53da8f66f50fb5cf38782c7/pidgin/win32/gtkwin32dep.c#lines-597

pub fn get_idle_time() -> Result<u64, Error> {

    let now = unsafe { GetTickCount() };

    let mut last_input_info = LASTINPUTINFO {
        cbSize: 8, // ! Probably only true for 64 bit systems?
        dwTime: 0
    };

    let p_last_input_info: PLASTINPUTINFO = &mut last_input_info as *mut LASTINPUTINFO;

    let ok = unsafe { GetLastInputInfo(p_last_input_info) } != 0;

    match ok {
        true => Ok((now - last_input_info.dwTime) / 1000),
        false => Err(Error::new("GetLastInputInfo failed"))
    }

}
