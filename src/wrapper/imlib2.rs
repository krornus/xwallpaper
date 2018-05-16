use std::ffi::CString;
use std::fmt;

use x11::{xlib};
use libc::{c_int};

use imlib2;
use xorg::XorgSession;

pub fn imlib_load_image_with_error_return(file: &str, err: &mut imlib2::Imlib_Load_Error) -> imlib2::Imlib_Image {
    let cfile = CString::new(file).unwrap();
    let err_ref = err as *mut imlib2::Imlib_Load_Error;

    unsafe { imlib2::imlib_load_image_with_error_return(cfile.as_ptr(), err_ref) }
}

pub fn imlib_context_set_display(display: *mut imlib2::Display) {
    unsafe { imlib2::imlib_context_set_display(display) }
}

pub fn imlib_context_set_visual(visual: *mut xlib::Visual) {
    unsafe { imlib2::imlib_context_set_visual(visual) }
}

pub fn imlib_context_set_colormap(colormap: xlib::Colormap) {
    unsafe { imlib2::imlib_context_set_colormap(colormap) }
}

pub fn _imlib_context_set_color_modifier(color_modifier: imlib2::Imlib_Color_Modifier) {
    unsafe { imlib2::imlib_context_set_color_modifier(color_modifier) }
}

pub fn _imlib_context_set_progress_function(progress_function: imlib2::Imlib_Progress_Function) {
    unsafe { imlib2::imlib_context_set_progress_function(progress_function) }
}

pub fn imlib_context_set_operation(operation: imlib2::Imlib_Operation) {
    unsafe { imlib2::imlib_context_set_operation(operation) }
}

pub fn imlib_set_cache_size(bytes: c_int) {
    unsafe { imlib2::imlib_set_cache_size(bytes) }
}

pub fn _imlib_create_image(width: c_int, height: c_int) -> imlib2::Imlib_Image {
    unsafe { imlib2::imlib_create_image(width, height) }
}

pub fn _imlib_context_set_image(image: imlib2::Imlib_Image) {
    unsafe { imlib2::imlib_context_set_image(image) }
}

pub fn imlib_image_set_changes_on_disk() {
    unsafe { imlib2::imlib_image_set_changes_on_disk() }
}

pub fn init_imlib2(xsess: &XorgSession, cache_size: i32) {
    imlib_context_set_display(xsess.disp);
    imlib_context_set_visual(xsess.visual);
    imlib_context_set_colormap(xsess.colormap);
    imlib_context_set_operation(0);
    imlib_set_cache_size(cache_size * 1024 * 1024);
}

#[derive(Debug)]
pub struct Image {
    pub image: imlib2::Imlib_Image,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(im: imlib2::Imlib_Image) -> Self {
        unsafe { imlib2::imlib_context_set_image(im) };

        let w = unsafe { imlib2::imlib_image_get_width() };
        let h = unsafe { imlib2::imlib_image_get_height() };

        Image {
            image: im,
            width: w as u32,
            height: h as u32,
        }
    }

    pub fn load<T: AsRef<str>>(file: T) -> Result<Self, imlib2::Imlib_Load_Error> {

        let mut err = 0;

        let im = imlib_load_image_with_error_return(file.as_ref(), &mut err);

        unsafe { imlib2::imlib_context_set_image(im) };

        if err != 0 {
            Err(err)
        } else {
            let w = unsafe { imlib2::imlib_image_get_width() };
            let h = unsafe { imlib2::imlib_image_get_height() };

            Ok(Image {
                image: im,
                width: w as u32,
                height: h as u32,
            })
        }
    }
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect {{ x: {}, y: {}, w: {}, h: {} }}", self.x, self.y, self.w, self.h)
    }
}


#[derive(Debug)]
pub struct DrawableContext {
    pub drawable: xlib::Drawable,
    pub dither: i8,
    pub blend: i8,
    pub alias: i8,
}

impl DrawableContext {
    pub fn new(d: xlib::Drawable, dither: i8, blend: i8, alias: i8) -> Self {
        DrawableContext {
            drawable: d,
            dither: dither,
            blend: blend,
            alias: alias,
        }
    }

    pub fn render_image(&self, im: &Image, size: Rect) {
        self.set_image_context(im.image);

        unsafe {
            imlib2::imlib_render_image_on_drawable_at_size(
                size.x, size.y, size.w, size.h);
        }
    }

    pub fn render_image_part(&self, im: &Image, part: Rect, size: Rect) {

        self.set_image_context(im.image);

        unsafe {
            imlib2::imlib_render_image_part_on_drawable_at_size(
                part.x, part.y, part.w, part.h,
                size.x, size.y, size.w, size.h);
        }
    }

    fn set_context(&self) {
        unsafe {
            imlib2::imlib_context_set_drawable(self.drawable);
            imlib2::imlib_context_set_anti_alias(self.alias);
            imlib2::imlib_context_set_dither(self.dither);
            imlib2::imlib_context_set_blend(self.blend);
            imlib2::imlib_context_set_angle(0_f64);
        }
    }

    fn set_image_context(&self, im: imlib2::Imlib_Image) {
        unsafe { imlib2::imlib_context_set_image(im) };
        self.set_context();
    }
}
