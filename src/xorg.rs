#![allow(non_snake_case)]
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;

use libc::{c_ulong,c_int,c_uint,c_uchar,c_long};
use x11::{xlib,xinerama};

pub trait Initialize {
    fn initialize() -> Self;
}

pub trait IsNull {
    fn is_null(&self) -> bool;
}

impl Initialize for xlib::XColor {
    fn initialize() -> Self {
        xlib::XColor {
            pixel: 0,
            red: 0,
            green: 0,
            blue: 0,
            flags: 0,
            pad: 0,
        }
    }
}

impl Initialize for u64 {
    fn initialize() -> Self {
        0
    }
}


impl Initialize for xlib::XGCValues {
    fn initialize() -> Self {
        xlib::XGCValues {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            tile: xlib::Pixmap::initialize(),
            stipple: xlib::Pixmap::initialize(),
            ts_x_origin: 0,
            ts_y_origin: 0,
            font: xlib::Font::initialize(),
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: xlib::Pixmap::initialize(),
            dash_offset: 0,
            dashes: 0,
        }
    }
}

pub struct XorgSession {
    pub disp: *mut xlib::Display,
    pub visual: *mut xlib::Visual,
    pub screen: *mut xlib::Screen,
    pub root: Window,
    pub depth: u32,
    pub colormap: u64,
}

pub struct Window {
    pub window: xlib::Window,
    display: *mut xlib::Display
}

impl Window {
    pub fn clear(&self) -> c_int {
        XClearWindow(self.display, self.window)
    }

    pub fn set_background_pixmap(&self, drawable: xlib::Pixmap) -> c_int {
        XSetWindowBackgroundPixmap(self.display, self.window, drawable)
    }

    pub fn change_property<T>(
        &self,
        property: xlib::Atom,
        property_type: xlib::Atom,
        format: c_int,
        mode: c_int,
        data: *const T,
        nelements: c_int
    ) -> c_int {
        XChangeProperty(
            self.display, self.window, property,
            property_type, format, mode, data, nelements
        )
    }

    pub fn property(
        &self,
        property: xlib::Atom,
        long_offset: c_long,
        long_length: c_long,
        delete: bool,
        req_type: xlib::Atom,
    ) -> WindowProperty {
        XGetWindowProperty(self.display, self.window, property, long_offset, long_length, delete, req_type)
    }

    pub fn pixmap(&self, w: u32, h: u32, depth: u32) -> xlib::Pixmap {
        XCreatePixmap(
            self.display, self.window,
            w, h, depth
        )
    }

    pub fn query_pointer(&self) -> Pointer {
        let mut root: c_ulong = 0;
        let mut child: c_ulong = 0;
        let mut rx: c_int = 0;
        let mut ry: c_int = 0;
        let mut wx: c_int = 0;
        let mut wy: c_int = 0;
        let mut mask: c_uint = 0;

        unsafe { xlib::XQueryPointer(self.display, self.window, &mut root, &mut child, &mut rx, &mut ry, &mut wx, &mut wy, &mut mask) };

        Pointer {
            root,
            child,
            rx,
            ry,
            wx,
            wy,
            mask,
        }
    }
}

impl Drop for XorgSession {
    fn drop(&mut self) {
        //self.close();
    }
}


impl XorgSession {
    pub fn default() -> Self {
        let disp = XOpenDisplay(None);

        let default_screen = XDefaultScreen(disp);

        let visual = XDefaultVisual(disp, default_screen);
        let root = XRootWindow(disp, default_screen);
        let depth = XDefaultDepth(disp, default_screen);
        let scr = XScreenOfDisplay(disp, default_screen);
        let cmap = XDefaultColormap(disp, default_screen);

        XorgSession {
            disp: disp,
            visual: visual,
            screen: scr,
            root: Window {
                window: root,
                display: disp,
            },
            depth: depth as u32,
            colormap: cmap,
        }
    }

    pub fn screen(&self) -> &xlib::Screen {
        unsafe { &*self.screen }
    }

    pub fn free_gc(&self, gc: xlib::GC) -> c_int {
        XFreeGC(self.disp, gc)
    }

    pub fn flush(&self) -> c_int {
        XFlush(self.disp)
    }

    pub fn close(&self) -> c_int {
        XCloseDisplay(self.disp)
    }

    pub fn fill_rectangle(&self,
        d: c_ulong,
        gc: xlib::GC,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint) -> c_int {
        XFillRectangle(self.disp, d, gc, x, y, width, height)
    }


    pub fn set_close_down_mode(&self, mode: c_int) -> c_int {
        XSetCloseDownMode(self.disp, mode)
    }

    pub fn free_pixmap(&self, drawable: xlib::Pixmap) -> c_int {
        XFreePixmap(self.disp, drawable)
    }

    pub fn kill_client(&self, resource: xlib::XID) -> c_int {
        XKillClient(self.disp, resource)
    }

    pub fn atom(&self, atom_name: &str, only_if_exists: bool) -> Option<xlib::Atom> {
        let atom = XInternAtom(self.disp, atom_name, only_if_exists);

        if atom == 0 {
            None
        } else {
            Some(atom)
        }
    }

    pub fn sync(&self, discard: bool) -> c_int {
        XSync(self.disp, discard)
    }


    pub fn gc(&self, drawable: c_ulong, valuemask: c_ulong, values: &mut xlib::XGCValues) -> xlib::GC {
        XCreateGC(
            self.disp, drawable,
            valuemask,
            values
        )
    }

    pub fn named_color(&self, name: &str) -> xlib::XColor {

        let mut color = xlib::XColor::initialize();

        XAllocNamedColorSame(
            self.disp,
            self.colormap,
            name, &mut color
        );

        color
    }
}

pub struct XineramaScreens<'a> {
    pub screens: Vec<&'a xinerama::XineramaScreenInfo>,
    pub count: i32,
    pub screen: i32,
}

impl XorgSession {
    pub fn xinerama_screens<'a>(&self) -> Option<XineramaScreens<'a>> {

        if XineramaIsActive(self.disp) {

            let mut _screens_ptr = ptr::null();
            let mut count: i32 = 0;
            let mut screen: i32 = 0;

            let pointer = self.root.query_pointer();
            let pos = Point { x: pointer.rx, y: pointer.ry };

            _screens_ptr = XineramaQueryScreens(self.disp, &mut count);
            let mut screens = Vec::with_capacity(count as usize);


            for i in 0..count {
                let info: &xinerama::XineramaScreenInfo = unsafe { &*_screens_ptr.offset(i as isize) };
                screens.push(info);

                if pos.in_rectangle(
                        info.x_org as i32,
                        info.y_org as i32,
                        info.width as i32,
                        info.height as i32) {
                    screen = i;
                }
            }

            Some(
                XineramaScreens {
                    screens,
                    count,
                    screen,
                }
            )
        } else {
            None
        }
    }
}


//TODO change rust types to libc?
pub fn XOpenDisplay(display: Option<&str>) -> *mut xlib::Display {
    if let Some(s) = display {
        let raw_str = CString::new(s).unwrap();
        unsafe { xlib::XOpenDisplay(raw_str.as_ptr()) }
    } else {
        unsafe { xlib::XOpenDisplay(ptr::null()) }
    }
}

pub fn XAllocNamedColorSame(display: *mut xlib::Display,
                        colormap: xlib::Colormap,
                        color_name: &str,
                        ret: &mut xlib::XColor)
-> c_int {
    let color_str = CString::new(color_name).unwrap();

    unsafe { xlib::XAllocNamedColor(
            display, colormap, color_str.as_ptr(),
            ret, ret
        )
    }
}

pub fn XDefaultVisual(display: *mut xlib::Display, scr: i32) -> *mut xlib::Visual {
    unsafe { xlib::XDefaultVisual(display, scr) }
}

pub fn XDefaultScreen(display: *mut xlib::Display) -> c_int {
    unsafe { xlib::XDefaultScreen(display) }
}

pub fn XRootWindow(display: *mut xlib::Display, scr: i32) -> c_ulong {
    unsafe { xlib::XRootWindow(display, scr) }
}

pub fn XDefaultDepth(display: *mut xlib::Display, scr: i32) -> c_int {
    unsafe { xlib::XDefaultDepth(display, scr) }
}

pub fn XScreenOfDisplay(display: *mut xlib::Display, scr: i32) -> *mut xlib::Screen {
    unsafe { xlib::XScreenOfDisplay(display, scr) }
}

pub fn XSync(display: *mut xlib::Display, discard: bool) -> c_int {
    unsafe { xlib::XSync(display, discard as c_int) }
}

pub fn XFreeGC(display: *mut xlib::Display, gc: xlib::GC) -> c_int {
    unsafe { xlib::XFreeGC(display, gc) }
}

pub fn XFree<T>(data: *const T) -> c_int {
    unsafe { xlib::XFree(data as *mut c_void) }
}

pub fn XFreePixmap(display: *mut xlib::Display, pixmap: xlib::Pixmap) -> c_int {
    unsafe { xlib::XFreePixmap(display, pixmap) }
}

pub fn XDefaultColormap(display: *mut xlib::Display, scr: i32) -> c_ulong {
    unsafe { xlib::XDefaultColormap(display, scr) }
}

pub fn XFillRectangle(display: *mut xlib::Display,
    d: c_ulong,
    gc: xlib::GC,
    x: c_int,
    y: c_int,
    width: c_uint,
    height: c_uint) -> c_int {

    unsafe { xlib::XFillRectangle(display, d, gc, x, y, width, height) }
}

pub fn XInternAtom(display: *mut xlib::Display, atom_name: &str, only_if_exists: bool)
    -> c_ulong {
    let atom_str = CString::new(atom_name).unwrap();
    unsafe { xlib::XInternAtom(display, atom_str.as_ptr(), only_if_exists as c_int) }
}

pub struct WindowProperty {
    pub property_type: xlib::Atom,
    pub format: c_int,
    pub count: c_ulong,
    pub bytes_after: c_ulong,
    pub property: *mut c_uchar,
}

pub fn XGetWindowProperty(
    display: *mut xlib::Display,
    w: xlib::Window,
    property: xlib::Atom,
    long_offset: c_long,
    long_length: c_long,
    delete: bool,
    req_type: xlib::Atom,
    )
    -> WindowProperty {

    let mut property_type: xlib::Atom = 0;
    let mut format: c_int = 0;
    let mut count: c_ulong = 0;
    let mut bytes_after: c_ulong =0;

    let cs = CString::new("").unwrap();
    let mut out_property: *mut c_uchar = cs.into_raw() as *mut c_uchar;


    unsafe { xlib::XGetWindowProperty(
        display,
        w,
        property,
        long_offset,
        long_length,
        delete as c_int,
        req_type,
        &mut property_type,
        &mut format,
        &mut count,
        &mut bytes_after,
        &mut out_property,
    )};


    WindowProperty {
        property_type,
        format,
        count,
        bytes_after,
        property: out_property,
    }
}

pub fn XCreatePixmap(display: *mut xlib::Display, scr: u64,
    w: u32, h: u32, depth: u32) -> c_ulong {

    unsafe { xlib::XCreatePixmap(display, scr, w, h, depth) }
}

pub fn XineramaIsActive(display: *mut xlib::Display) -> bool {
    unsafe { xinerama::XineramaIsActive(display) != 0 }
}

pub fn XineramaQueryScreens(display: *mut xlib::Display, num: &mut c_int)
    -> *mut xinerama::XineramaScreenInfo {
    unsafe { xinerama::XineramaQueryScreens(display, num) }
}

pub fn XCreateGC(display: *mut xlib::Display, d: c_ulong,
    valuemask: c_ulong, values: *mut xlib::XGCValues
) -> xlib::GC {
    unsafe { xlib::XCreateGC(display, d, valuemask, values) }
}

pub fn XKillClient(display: *mut xlib::Display, resource: xlib::XID) -> c_int {
    unsafe { xlib::XKillClient(display,resource) }
}

pub fn XSetWindowBackgroundPixmap(display: *mut xlib::Display, w: xlib::Window, bg_pixmap: xlib::Pixmap) -> c_int {
    unsafe { xlib::XSetWindowBackgroundPixmap(display, w, bg_pixmap) }
}

pub fn XClearWindow(display: *mut xlib::Display, w: xlib::Window) -> c_int {
    unsafe { xlib::XClearWindow(display, w) }
}

pub fn XFlush(display: *mut xlib::Display) -> c_int {
    unsafe { xlib::XFlush(display) }
}

pub fn XCloseDisplay(display: *mut xlib::Display) -> c_int {
    unsafe { xlib::XCloseDisplay(display) }
}
pub fn XSetCloseDownMode(display: *mut xlib::Display, close_mode: c_int) -> c_int {
    unsafe { xlib::XSetCloseDownMode(display,close_mode) }
}

pub fn XChangeProperty<T>(display: *mut xlib::Display,
    w: xlib::Window,
    property: xlib::Atom,
    property_type: xlib::Atom,
    format: c_int,
    mode: c_int,
    data: *const T,
    nelements: c_int
) -> c_int {
    unsafe {
        xlib::XChangeProperty(
            display,
            w,
            property,
            property_type,
            format,
            mode,
            data as *const c_uchar,
            nelements,
        )
    }
}

pub struct Pointer {
    pub root: xlib::Window,
    pub child: xlib::Window,
    pub rx: c_int,
    pub ry: c_int,
    pub wx: c_int,
    pub wy: c_int,
    pub mask: c_uint,
}

impl Pointer {
    pub fn query(display: *mut xlib::Display, window: c_ulong) -> Self {
        let mut root: c_ulong = 0;
        let mut child: c_ulong = 0;
        let mut rx: c_int = 0;
        let mut ry: c_int = 0;
        let mut wx: c_int = 0;
        let mut wy: c_int = 0;
        let mut mask: c_uint = 0;

        unsafe { xlib::XQueryPointer(display, window, &mut root, &mut child, &mut rx, &mut ry, &mut wx, &mut wy, &mut mask) };

        Pointer {
            root,
            child,
            rx,
            ry,
            wx,
            wy,
            mask,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    fn in_rectangle(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        (self.x >= x) && (self.y >= y) && (self.x <= (x + w)) && (self.y <= (y + h))
    }
}
