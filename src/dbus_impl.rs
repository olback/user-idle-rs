use std::time::Duration;

use dbus::blocking::Connection;

use crate::error::Error;

// Based on https://bitbucket.org/pidgin/main/src/default/pidgin/gtkidle.c

const SCREENSAVERS: &[&[&str]] = &[
    &[
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        "org.freedesktop.ScreenSaver",
    ],
    &[
        "org.gnome.ScreenSaver",
        "/org/gnome/ScreenSaver",
        "org.gnome.ScreenSaver",
    ],
    &[
        "org.kde.ScreenSaver",
        "/org/kde/ScreenSaver",
        "org.kde.ScreenSaver",
    ],
];

pub fn get_idle_time() -> Result<Duration, Error> {
    for screensaver in SCREENSAVERS {
        let Ok(conn) = Connection::new_session() else {continue};

        let proxy = conn.with_proxy(
            screensaver[0],
            screensaver[1],
            Duration::from_millis(5000),
        );

        let (time,): (u32,) =
            match proxy.method_call(screensaver[2], "GetActiveTime", ()) {
                Ok(v) => v,
                Err(_) => continue,
            };

        // freedesktop seems to return the time in milliseconds??
        if screensaver[0] == "org.freedesktop.ScreenSaver" {
            return Ok(Duration::from_millis(u64::from(time)));
        }

        return Ok(Duration::from_secs(u64::from(time)));
    }

    Err(Error::new("No screensaver available"))
}
