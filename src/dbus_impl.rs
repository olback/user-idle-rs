use crate::error::Error;
use dbus;

// Based on https://bitbucket.org/pidgin/main/src/default/pidgin/gtkidle.c

const SCREENSAVERS: &'static [&'static [&'static str]] = &[
    &["org.freedesktop.ScreenSaver", "/org/freedesktop/ScreenSaver", "org.freedesktop.ScreenSaver"],
    &["org.gnome.ScreenSaver", "/org/gnome/ScreenSaver", "org.gnome.ScreenSaver"],
    &["org.kde.ScreenSaver", "/org/kde/ScreenSaver", "org.kde.ScreenSaver"]
];

pub fn get_idle_time() -> Result<u64, Error> {

    for screensaver in SCREENSAVERS {

        let conn = match dbus::blocking::Connection::new_session() {
            Ok(conn) => conn,
            Err(_) => continue
        };

        let proxy = conn.with_proxy(screensaver[0], screensaver[1], std::time::Duration::from_millis(5000));

        let (time,): (u32,) = match proxy.method_call(screensaver[2], "GetActiveTime", ()) {
            Ok(v) => v,
            Err(_) => continue
        };

        // freedesktop seems to return the time in milliseconds??
        if screensaver[0] == "org.freedesktop.ScreenSaver" {
            return Ok((time / 1000) as u64)
        }

        return Ok(time as u64)

    }

    Err(Error::new("No screensaver available"))

}
