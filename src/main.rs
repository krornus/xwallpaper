extern crate libc;
extern crate x11;
extern crate imlib2;

use x11::xlib;
use libc::c_ulong;

mod wrapper;
mod xorg;
mod render;

use xorg::{Initialize};

fn set_background() {

    let xsess = xorg::XorgSession::default();
    let scr = xsess.screen();
    let (width,height) = (scr.width as u32, scr.height as u32);

    wrapper::imlib2::init_imlib2(&xsess, 4);

    let drawable = xsess.root.pixmap(width, height, xsess.depth);
    let color = xsess.named_color("black");

    bg_fill(&xsess, drawable, color);

    let wallpaper = render::Wallpaper::load(
        "/home/spowell/pictures/backgrounds/darkstar_poster.jpg", drawable
    ).unwrap();
    wallpaper.render(scr, render::Mode::Center);


    let xsess2 = xorg::XorgSession::default();
    xsess.sync(false);

    let drawable2 = xsess2.root.pixmap(width, height, xsess2.depth);

    let mut gcvalues = xlib::XGCValues::initialize();
    gcvalues.fill_style = xlib::FillTiled;
    gcvalues.tile = drawable;

    let gc = xsess2.gc(drawable2,
        (xlib::GCFillStyle | xlib::GCTile) as u64,
        &mut gcvalues
    );

    xsess2.fill_rectangle(
        drawable2, gc, 0, 0,
        width, height
    );

    xsess2.free_gc(gc);

    xsess.sync(false);
    xsess2.sync(false);

    xsess.free_pixmap(drawable);

    let prop_root = xsess2.atom("_XROOTPMAP_ID", true)
        .expect("Failed to create pixmap property");

    let prop_esetroot = xsess2.atom("ESETROOT_PMAP_ID", true)
        .expect("Failed to create pixmap property");

    let root_pmap_id = xsess2.root.property(
        prop_root, 0, 1, false,
        xlib::AnyPropertyType as u64
    );

    if root_pmap_id.property_type == xlib::XA_PIXMAP {

        let esetroot_pmap_id = xsess2.root.property(
            prop_esetroot, 0, 1, false,
            xlib::AnyPropertyType as u64
        );

        if !root_pmap_id.property.is_null() && !esetroot_pmap_id.property.is_null() {
            if esetroot_pmap_id.property_type == xlib::XA_PIXMAP {

                let pm1 = unsafe { *root_pmap_id.property as xlib::Pixmap };
                let pm2 = unsafe { *esetroot_pmap_id.property as xlib::Pixmap };

                if pm1 == pm2 {
                    xsess2.kill_client(pm1);
                }
            }
        }

        if !esetroot_pmap_id.property.is_null() {
            xorg::XFree(root_pmap_id.property);
        }
    }

    if !root_pmap_id.property.is_null() {
        xorg::XFree(root_pmap_id.property);
    }

    xsess2.root.change_property(
        prop_root, xlib::XA_PIXMAP,
        32, xlib::PropModeReplace,
        &drawable2, 1
    );

    xsess2.root.change_property(
        prop_esetroot, xlib::XA_PIXMAP,
        32, xlib::PropModeReplace,
        &drawable2, 1
    );

    xsess2.root.set_background_pixmap(drawable2);
    xsess2.root.clear();
    xsess2.flush();
    xsess2.set_close_down_mode(xlib::RetainPermanent);
    xsess2.free_pixmap(drawable2);
    /* closing the display causes infinite hang? */
}

fn bg_fill(xsess: &xorg::XorgSession, drawable: c_ulong, color: xlib::XColor) {
    let mut gcval = xlib::XGCValues::initialize();
    let scr = xsess.screen();

    gcval.foreground = color.pixel;

    let gc = xsess.gc(
        xsess.root.window,
        (xlib::GCForeground) as u64,
        &mut gcval
    );

    xsess.fill_rectangle(
        drawable, gc, 0, 0,
        scr.width as u32, scr.height as u32
    );

    xsess.free_gc(gc);
}

fn main() {
    set_background();
}
