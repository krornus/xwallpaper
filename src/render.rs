use std::ffi::OsStr;

use x11::xlib;

use image;
use imlib2::Imlib_Load_Error;

#[derive(Debug, Clone)]
pub enum Mode {
    Scale,
    Center,
    Fill,
    Max,
}

#[derive(Debug)]
pub struct Wallpaper {
    drawable: image::DrawableContext,
    image: image::Image,
}

impl Wallpaper {
    pub fn load<T: AsRef<OsStr> + Sized>(
        path: T,
        drawable: xlib::Drawable,
    ) -> Result<Self, Imlib_Load_Error> {
        let im = image::Image::load(path)?;
        let ctx = image::DrawableContext::new(drawable, 1, 1, 0);

        Ok(Wallpaper {
            drawable: ctx,
            image: im,
        })
    }

    pub fn render_at(&self, mode: Mode, area: image::Rect) {
        match mode {
            Mode::Scale => self.render_scale(area),
            Mode::Center => self.render_center(area),
            Mode::Fill => self.render_fill(area),
            Mode::Max => self.render_max(area),
        }
    }

    fn render_scale(&self, area: image::Rect) {
        let size = image::Rect {
            x: area.x,
            y: area.y,
            w: area.w,
            h: area.h,
        };

        self.drawable.render_image(&self.image, size);
    }

    fn render_center(&self, area: image::Rect) {
        let cx: i32 = (area.w - self.image.width as i32) / 2;
        let cy: i32 = (area.h - self.image.height as i32) / 2;

        // true if image wider than screen
        let xltz = cx < 0;
        // true if image taller than screen
        let yltz = cy < 0;

        let part = image::Rect {
            x: -cx * (xltz as i32),
            y: -cy * (yltz as i32),
            w: area.w,
            h: area.h,
        };

        let size = image::Rect {
            x: cx * (!xltz as i32),
            y: cy * (!yltz as i32),
            w: area.w,
            h: area.h,
        };

        self.drawable.render_image_part(&self.image, part, size);
    }

    fn render_fill(&self, area: image::Rect) {
        let (img_w, img_h) = (self.image.width as i32, self.image.height as i32);
        let (part, size): (image::Rect, image::Rect);

        let cut_x = (img_w * area.h) > (img_h * area.w);

        if cut_x {
            let w = (img_h * area.w) / area.h;
            let h = img_h;
            let x = (img_w - w) / 2;
            let y = 0;

            part = image::Rect { x, y, w, h };
        } else {
            let w = img_w;
            let h = (img_w * area.h) / area.w;
            let x = 0;
            let y = (img_h - h) / 2;

            part = image::Rect { x, y, w, h };
        }

        size = image::Rect {
            x: 0,
            y: 0,
            w: area.w,
            h: area.h,
        };

        self.drawable.render_image_part(&self.image, part, size);
    }

    fn render_max(&self, area: image::Rect) {
        let (img_w, img_h) = (self.image.width as i32, self.image.height as i32);

        let (x, y, w, h): (i32, i32, i32, i32);

        if (img_w * area.h) > (img_h * area.w) {
            w = area.w;
            h = (img_h * area.w) / img_w;
            x = area.x;
            y = area.y + (area.h - h) / 2;
        } else {
            w = (img_w * area.h) / img_h;
            h = area.h;
            x = area.x + (area.w - w) / 2;
            y = area.y;
        }

        let size = image::Rect { x, y, w, h };

        self.drawable.render_image(&self.image, size);
    }
}
