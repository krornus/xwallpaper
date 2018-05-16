mod import {
    use x11::xlib;
    use libc::{c_int,c_char};

    use imlib2;

    #[link(name = "giblib")]
    extern {
        pub fn gib_imlib_render_image_on_drawable_at_size(d: xlib::Drawable, im: imlib2::Imlib_Image,
                                                    x: c_int, y: c_int,
                                                    w: c_int, h: c_int, dither: c_char,
                                                    blend: c_char, alias: c_char);

        pub fn gib_imlib_image_get_width(im: imlib2::Imlib_Image) -> c_int;
        pub fn gib_imlib_image_get_height(im: imlib2::Imlib_Image) -> c_int;
        pub fn gib_imlib_render_image_part_on_drawable_at_size(d: xlib::Drawable, im: imlib2::Imlib_Image,
                                                    sy: c_int, sw: c_int, sh: c_int,
                                                    x: c_int, y: c_int,
                                                    w: c_int, h: c_int, dither: c_char,
                                                    blend: c_char, alias: c_char);
    }
}


use x11::xlib;
use libc::{c_int,c_char};

use imlib2;

pub fn gib_imlib_render_image_on_drawable_at_size(d: xlib::Drawable, im: imlib2::Imlib_Image,
                                            x: c_int, y: c_int,
                                            w: c_int, h: c_int, dither: c_char,
                                            blend: c_char, alias: c_char)
    {
    unsafe { import::gib_imlib_render_image_on_drawable_at_size(d,im,x,y,w,h,dither,blend,alias) }
}

pub fn gib_imlib_render_image_part_on_drawable_at_size(d: xlib::Drawable, im: imlib2::Imlib_Image,
                                                    sy: c_int, sw: c_int, sh: c_int,
                                                    x: c_int, y: c_int,
                                                    w: c_int, h: c_int, dither: c_char,
                                                    blend: c_char, alias: c_char)
    {
    unsafe { import::gib_imlib_render_image_part_on_drawable_at_size(d,im,sy,sw,sh,x,y,w,h,dither,blend,alias) }
}

pub fn gib_imlib_image_get_width(im: imlib2::Imlib_Image) -> c_int {
    unsafe { import::gib_imlib_image_get_width(im) }
}

pub fn gib_imlib_image_get_height(im: imlib2::Imlib_Image) -> c_int {
    unsafe { import::gib_imlib_image_get_height(im) }
}
