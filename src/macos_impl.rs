use std::{io, mem::size_of, ptr::null_mut, time::Duration};

use mach::{
    kern_return::KERN_SUCCESS,
    port::{mach_port_t, MACH_PORT_NULL},
};
use CoreFoundation_sys::{
    kCFAllocatorDefault, kCFNumberSInt64Type, kCFStringEncodingUTF8,
    CFDataGetBytes, CFDataGetTypeID, CFDictionaryGetValueIfPresent,
    CFGetTypeID, CFNumberGetTypeID, CFNumberGetValue, CFRange, CFRelease,
    CFStringCreateWithCString, CFTypeRef,
};
use IOKit_sys::{
    IOIteratorNext, IOMasterPort, IOObjectRelease,
    IORegistryEntryCreateCFProperties, IOServiceGetMatchingServices,
    IOServiceMatching,
};

use crate::error::Error;

pub fn get_idle_time() -> Result<Duration, Error> {
    let mut ns = 0u64;
    let mut port: mach_port_t = 0;
    let mut iter = 0;
    let mut value: CFTypeRef = null_mut();
    let mut properties = null_mut();
    let entry;

    unsafe {
        let port_result = IOMasterPort(MACH_PORT_NULL, &mut port as _);
        if port_result != KERN_SUCCESS {
            return Err(Error {
                cause: format!(
                    "Unable to open mach port: {}",
                    io::Error::last_os_error()
                ),
            });
        }

        let service_name = cstr::cstr!("IOHIDSystem");
        let service_result = IOServiceGetMatchingServices(
            port as _,
            IOServiceMatching(service_name.as_ptr() as _),
            &mut iter,
        );
        if service_result != KERN_SUCCESS {
            return Err(Error {
                cause: format!(
                    "Unable to lookup IOHIDSystem: {}",
                    io::Error::last_os_error()
                ),
            });
        }

        if iter > 0 {
            entry = IOIteratorNext(iter);
            if entry > 0 {
                let prop_res = IORegistryEntryCreateCFProperties(
                    entry,
                    &mut properties as _,
                    kCFAllocatorDefault,
                    0,
                );

                if prop_res == KERN_SUCCESS {
                    let prop_name = cstr::cstr!("HIDIdleTime");
                    let prop_name_cf = CFStringCreateWithCString(
                        kCFAllocatorDefault,
                        prop_name.as_ptr() as _,
                        kCFStringEncodingUTF8,
                    );
                    let present = CFDictionaryGetValueIfPresent(
                        properties,
                        prop_name_cf as _,
                        &mut value,
                    );
                    CFRelease(prop_name_cf.cast());

                    if present == 1 {
                        IOObjectRelease(iter);
                        IOObjectRelease(entry as _);
                        CFRelease(properties as _);
                        if CFGetTypeID(value) == CFDataGetTypeID() {
                            let mut buf = [0u8; size_of::<i64>()];
                            let range = CFRange {
                                location: buf.as_ptr() as _,
                                length: size_of::<i64>() as _,
                            };
                            CFDataGetBytes(value as _, range, buf.as_mut_ptr());
                            ns = i64::from_ne_bytes(buf) as u64;
                        } else if CFGetTypeID(value) == CFNumberGetTypeID() {
                            let mut buf = [0i64, 1];
                            CFNumberGetValue(
                                value as _,
                                kCFNumberSInt64Type,
                                buf.as_mut_ptr() as _,
                            );
                            ns = buf[0] as u64;
                        }
                    }
                }
            }
            IOObjectRelease(entry as _);
        }
        IOObjectRelease(iter);
    }

    Ok(Duration::from_nanos(ns))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_panics() {
        assert!(get_idle_time().is_ok());
    }
}
