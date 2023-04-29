use std::time::Duration;

use crate::error::Error;

use IOKit_sys as io_kit;
use CoreFoundation_sys as cf;

use io_kit::IOMasterPort;
use mach::{kern_return::KERN_SUCCESS, port::{MACH_PORT_NULL, mach_port_t}};
use cf::{CFDataGetBytes, CFDataGetTypeID, CFDictionaryGetValueIfPresent, CFGetTypeID, CFNumberGetTypeID, CFNumberGetValue, CFRange, CFRelease, CFStringCreateWithCString, CFTypeRef, kCFAllocatorDefault, kCFNumberSInt64Type, kCFStringEncodingUTF8}; 

pub fn get_idle_time() -> Result<Duration, Error> {
    let mut ns = 0u64;
    let mut port: mach_port_t = 0;
    let mut iter = 0;
    let mut value: CFTypeRef = std::ptr::null_mut();
    let mut properties = std::ptr::null_mut();
    let entry;
    
    unsafe {
        let port_result = IOMasterPort(MACH_PORT_NULL, &mut port as _);
        if port_result != KERN_SUCCESS {
            let last_os = std::io::Error::last_os_error();
            return Err(Error {
                cause: format!("Unable to open mach port: {}", last_os),
            });
        }
        let service_name = cstr::cstr!("IOHIDSystem");
        let service_result = io_kit::IOServiceGetMatchingServices(
                port as _,
                io_kit::IOServiceMatching(service_name.as_ptr() as _),
                &mut iter,
            );
        if service_result != KERN_SUCCESS {
            let last_os = std::io::Error::last_os_error();
            return Err(Error {
                cause: format!("Unable to lookup IOHIDSystem: {}", last_os),
            });
        }
        if iter > 0 {
            entry = io_kit::IOIteratorNext(iter);
            if entry > 0 {
                let prop_res = io_kit::IORegistryEntryCreateCFProperties(
                    entry,
                    &mut properties as _,
                    kCFAllocatorDefault,
                    0,
                );
                
                if prop_res == KERN_SUCCESS {
                    let prop_name = cstr::cstr!("HIDIdleTime");
                    let prop_name_cf = CFStringCreateWithCString(kCFAllocatorDefault, prop_name.as_ptr() as _, kCFStringEncodingUTF8);
                    let present = CFDictionaryGetValueIfPresent(
                        properties,
                        prop_name_cf as _,
                        &mut value,
                    );

                    CFRelease(prop_name_cf.cast());

                    if present == 1 {
                        io_kit::IOObjectRelease(iter);
                        io_kit::IOObjectRelease(entry as _);
                        CFRelease(properties as _);
                        if CFGetTypeID(value) == CFDataGetTypeID() {
                            let mut buf = [0u8; std::mem::size_of::<i64>()];
                            let range = CFRange {
                                location: buf.as_ptr() as _,
                                length: std::mem::size_of::<i64>() as _,
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
            io_kit::IOObjectRelease(entry as _);
        }
        io_kit::IOObjectRelease(iter);
    }
    let dur = std::time::Duration::from_nanos(ns);
    Ok(dur)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_panics() {
        get_idle_time().unwrap();
    }
}
