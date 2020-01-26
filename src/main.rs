// Muck up X11.  This program will send a bunch of crap to X11 windows.
// Why?  We want to see what breaks, so we can fix it.  We're the good
// guys.

#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

// #include <x11>
extern crate x11;

use std::ptr;
use x11::xlib;

#[cfg(not(feature = "xlib"))]
fn main() {
    panic!("NOOOOOOOOOOOOOOOOOOOOOOOO");
}

#[cfg(feature = "xlib")]
fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed");
        }
    }
}
