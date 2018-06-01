extern crate libc;
extern crate x11;
extern crate imlib2;

use x11::xlib;

mod wrapper;
mod xorg;
mod render;

use xorg::Initialize;

fn set_background() {

    let xsess = xorg::XorgSession::default();
    let xsess2 = xorg::XorgSession::default();
    let scr = xsess.screen();

    wrapper::imlib2::init_imlib2(&xsess, 4);

    let color = xsess.named_color("black");
    let drawable = xsess.root.pixmap(1920,1080,xsess.depth);

    let wallpaper = render::Wallpaper::load(
        "/home/spowell/pictures/backgrounds/darkstar_poster.jpg", drawable
    ).unwrap();


    xsess.sync(false);

    let drawable2 = xsess.root.pixmap(scr.width as u32, scr.height as u32, xsess.depth);

    let mut gcvalues = xlib::XGCValues::initialize();
    gcvalues.fill_style = xlib::FillTiled;
    gcvalues.tile = drawable;
    gcvalues.foreground = color.pixel;

    let gc = xsess2.gc(drawable2,
        (xlib::GCFillStyle | xlib::GCTile) as u64,
        &mut gcvalues
    );

    xsess2.fill_rectangle(
        drawable, gc, 0, 0,
        scr.width as u32, scr.height as u32
    );

    //xsess2.free_gc(gc);
    xsess.sync(false);
    xsess2.sync(false);

    wallpaper.render(scr, render::Mode::Center);
    wrapper::imlib2::imlib_image_set_changes_on_disk();

    //xsess.free_pixmap(drawable);

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
    /* closing the display causes infinite hang? */
}

fn main() {
    set_background();
}
