// Muck up X11.  This program will send a bunch of crap to X11 windows.
// Why?  We want to see what breaks, so we can fix it.  We're the good
// guys.
//
// Some code taken from:
// https://github.com/erlepereira/x11-rs/tree/master/x11/examples

#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

// #include <x11>
extern crate x11;

//use std::os::raw::*;

fn main() {
    unsafe {
        let display = x11::xlib::XOpenDisplay(std::ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed");
        }
        let screen = x11::xlib::XDefaultScreen(display);
        let root = x11::xlib::XRootWindow(display, screen);

        // The type of parent is infered from the return value of the right hand operand
        let mut parent = root.clone(); //x11::xlib::XRootWindow(display, screen);
        // The type of children is a mutable raw pointer (*mut) to the parent.
        // The value which is being assigned to it is the address of parent.
        let mut children: *mut u64 = &mut parent;
        let mut nchildren: u32 = 0;

        // Get list of windows
        // https://tronche.com/gui/x/xlib/window-information/XQueryTree.html
        let mut status = x11::xlib::XQueryTree(display, root, &mut parent as *mut u64, &mut parent as *mut u64, &mut children as *mut _, &mut nchildren);
        // The above line of code has a lot going on.
        // The "&mut parent as *mut u64" is passing in a pointer to parent.
        // The "&mut parent" gets a mutable reference to a parent, then the
        // "as *mut u64" casts it into a mutable raw pointer... of size u64...
        // I think...  Does that mean this code won't work on an i386 system?
        //
        // Similarly, a pointer to children is being created here in the
        // same way, but since children is a pointer itself, that gets us the
        // pointer to a pointer that XQueryTree wants.  Whew!  Specifying
        // an underscore instead of a u64 seems to be required when casting
        // to a pointer to a pointer, although the Rust documentation doesn't
        // seem to have any details about what the size after a pointer is,
        // it just gives examples without any explanation :-(

        println!("XQueryTree returned {}", status);
        println!("root = {}", root);
        println!("nchildren = {}", nchildren);

        // I sure wish it were easier to create an empty structure in Rust...
        let mut attrs = x11::xlib::XWindowAttributes{x: 0, y: 0, width: 0, height: 0, border_width:
            0, depth: 0, visual: std::ptr::null_mut(), root: 0, class: 0, bit_gravity: 0,
            win_gravity: 0, backing_store: 0, backing_planes: 0, backing_pixel: 0, save_under: 0,
            colormap: 0, map_installed: 0, map_state: 0, all_event_masks: 0, your_event_mask: 0,
            do_not_propagate_mask: 0, override_redirect: 0, screen: std::ptr::null_mut()};

        for i in 0..nchildren-1 {
            let w = *(children.offset(i as isize));
            status = x11::xlib::XGetWindowAttributes(display, w, &mut attrs);
            //println!("XGetWindowAttributes[{}] returned {}", i, status);
            //println!("attrs.x = {}", attrs.x);
            //println!("attrs.y = {}", attrs.y);
            //println!("attrs.width = {}", attrs.width);
            //println!("attrs.height = {}", attrs.height);

            // https://stackoverflow.com/questions/58158069/how-do-i-create-uninitialized-pointers-that-i-can-pass-to-ffi-functions-in-a-thr
            let mut name: *mut i8 = std::ptr::null_mut();
            let nameptr: *mut *mut i8 = &mut name;
            // https://tronche.com/gui/x/xlib/ICC/client-to-window-manager/XFetchName.html
            status = x11::xlib::XFetchName(display, w, nameptr);
            //println!("XFetchName[{}] returned {}", i, status);
            if status != 0 {
                println!("Window {} name = {}", i, std::ffi::CString::from_raw(name).into_string().expect(""));
            }

            // https://tronche.com/gui/x/xlib/window-information/XGetWindowProperty.html
            //status = x11::xlib::XGetWindowProperty(display, w, property, long_offset, long_length, delete, req_type,
            //            actual_type_return, actual_format_return, nitems_return, bytes_after_return,
            //            prop_return)
        }


        /*
        // Create window.
        let screen = x11::xlib::XDefaultScreen(display);
        let root = x11::xlib::XRootWindow(display, screen);

        let mut attributes: x11::xlib::XSetWindowAttributes = std::mem::uninitialized();
        attributes.background_pixel = x11::xlib::XWhitePixel(display, screen);

        let window = x11::xlib::XCreateWindow(display, root,
                                         0, 0, 400, 300,
                                         0, 0,
                                         x11::xlib::InputOutput as c_uint, std::ptr::null_mut(),
                                         x11::xlib::CWBackPixel, &mut attributes);

        // Set window title.
        let title_str = std::ffi::CString::new("hello-world").unwrap();
        x11::xlib::XStoreName(display, window, title_str.as_ptr() as *mut c_char);
        // Hook close requests.
        let wm_protocols_str = std::ffi::CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = std::ffi::CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = x11::xlib::XInternAtom(display, wm_protocols_str.as_ptr(), x11::xlib::False);
        let wm_delete_window = x11::xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), x11::xlib::False);

        let mut protocols = [wm_delete_window];

        x11::xlib::XSetWMProtocols(display, window, protocols.as_mut_ptr(), protocols.len() as c_int);

        // Show window.
        x11::xlib::XMapWindow(display, window);

        // Main loop.
        let mut event: x11::xlib::XEvent = std::mem::uninitialized();

        loop {
            x11::xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                x11::xlib::ClientMessage => {
                    let xclient = x11::xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as x11::xlib::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                },

                _ => ()
            }
        }
        */

        // Shut down.
        x11::xlib::XCloseDisplay(display);
    }
}
