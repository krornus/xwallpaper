extern crate libc;
extern crate x11;
extern crate imlib2;

use x11::xlib;
use libc::c_ulong;

mod imlib2_wrapper;
mod xorg;
mod render;

use xorg::{Initialize};
use imlib2_wrapper::{AsRect};

/*
 * https://en.wikipedia.org/wiki/Pseudo-transparency
 * These properties are used to inform the window where it
 * can find the pixmap used on the root window. Using this
 * information, a client can paint a section of the image
 * (corresponding to the size and position of the window)
 * onto its background, achieving the effect of transparency.
 * This method uses the most memory, but has the advantage of
 * allowing clients to perform any operation, such as tinting
 * or shading, on the image data.
 */
struct RootWindowProperties {
    root_pixmap: Option<xlib::Atom>,
    esetroot_pixmap: Option<xlib::Atom>,
}

impl RootWindowProperties {

    fn new(xsess: &xorg::XorgSession) -> Self {
        RootWindowProperties {
            root_pixmap: xsess.atom("_XROOTPMAP_ID", true),
            esetroot_pixmap: xsess.atom("ESETROOT_PMAP_ID", true),
        }
    }

    /*
     * http://search.cpan.org/dist/X11-Protocol-Other/lib/X11/Protocol/XSetRoot.pm#ROOT_WINDOW_PROPERTIES
     * if the background is replaced, kill esetroot_pmap_id if it is the same as the root pixmap
     */
    fn cleanse_esetroot(&self, xsess: &xorg::XorgSession) {

        let prop_root_pmap = match self.root_pixmap {
            Some(pm) => pm,
            None => { return; },
        };

        let prop_esetroot_pmap = match self.esetroot_pixmap {
            Some(pm) => pm,
            None => { return; },
        };

        let root_pmap_id = xsess.root.property(
            prop_root_pmap, 0, 1, false,
            xlib::AnyPropertyType as u64
        );

        if root_pmap_id.property_type == xlib::XA_PIXMAP {

            let esetroot_pmap_id = xsess.root.property(
                prop_esetroot_pmap, 0, 1, false,
                xlib::AnyPropertyType as u64
            );

            if !root_pmap_id.property.is_null() && !esetroot_pmap_id.property.is_null() {
                if esetroot_pmap_id.property_type == xlib::XA_PIXMAP {

                    let pm1 = unsafe { *root_pmap_id.property as xlib::Pixmap };
                    let pm2 = unsafe { *esetroot_pmap_id.property as xlib::Pixmap };

                    /* kill if equal */
                    if pm1 == pm2 {
                        xsess.kill_client(pm1);
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
    }

    /* change the properties that store the current background as pixmap */
    pub fn update_background(&self, xsess: &xorg::XorgSession, drawable: c_ulong) {

        self.cleanse_esetroot(xsess);

        if let Some(root_pixmap) = self.root_pixmap {
            xsess.root.change_property(
                root_pixmap, xlib::XA_PIXMAP,
                32, xlib::PropModeReplace,
                &drawable, 1
            );
        }

        if let Some(esetroot_pixmap) = self.esetroot_pixmap {
            xsess.root.change_property(
                esetroot_pixmap, xlib::XA_PIXMAP,
                32, xlib::PropModeReplace,
                &drawable, 1
            );
        }
    }
}

enum ScreenOption {
    All,
    Active,
    Index(usize),
}

enum ColorOption<T: AsRef<str>> {
    Named(T),
    Hex(T),
    Default,
}

struct BackgroundOptions<T: AsRef<str>> {
    path: T,
    mode: render::Mode,
    bgcolor: ColorOption<T>,
    screen: ScreenOption,
}

impl<T: AsRef<str>> BackgroundOptions<T> {
    fn new(path: T, mode: render::Mode) -> Self {
        BackgroundOptions {
            path,
            mode,
            bgcolor: ColorOption::Default,
            screen: ScreenOption::All,
        }
    }

    pub fn set_background(&self) {

        let xsess  = xorg::XorgSession::default();
        let xsess2 = xorg::XorgSession::default();

        let xinerama = xsess.xinerama_screens()
            .expect("Error: no screens found with xinerama");

        let scr = xsess.screen();
        let (width,height) = (scr.width as u32, scr.height as u32);

        imlib2_wrapper::init_imlib2(&xsess, 4);

        let drawable = xsess.root.pixmap(width, height, xsess.depth);

        let color = match &self.bgcolor {
            ColorOption::Default => xsess.named_color("black"),
            ColorOption::Named(n) => xsess.named_color(n),
            ColorOption::Hex(h) => xsess.parse_color(h),
        };

        /* TODO: Add check to see if this is necessary or filled over */
        bg_fill(&xsess, drawable, color);

        /* TODO: dont just expect */
        let wallpaper = render::Wallpaper::load(
            self.path.as_ref(), drawable
        ).expect("failed to load wallpaper");

        match self.screen {
            ScreenOption::All => {
                for scr_inf in xinerama.screens.iter() {
                    wallpaper.render_at(self.mode.clone(), scr_inf.as_rect());
                }
            },
            ScreenOption::Active => {
                /* TODO: What if None?? */
                if let Some(scr_inf) = xinerama.active_screen() {
                    wallpaper.render_at(self.mode.clone(), scr_inf.as_rect());
                }
            },
            ScreenOption::Index(i) => {
                if i < xinerama.screens.len() {
                    let scr_inf = xinerama.screens[i];
                    wallpaper.render_at(self.mode.clone(), scr_inf.as_rect());
                }

            },
        }

        xsess.sync(false);

        let drawable2 = xsess2.root.pixmap(width, height, xsess2.depth);

        tile_drawable(&xsess2, drawable, drawable2);

        xsess.sync(false);
        xsess2.sync(false);

        xsess.free_pixmap(drawable);

        let root_props = RootWindowProperties::new(&xsess2);
        root_props.update_background(&xsess2, drawable2);

        xsess2.root.set_background_pixmap(drawable2);
        xsess2.root.clear();
        xsess2.flush();

        /* must cleanse esetroot */
        xsess2.set_close_down_mode(xlib::RetainPermanent);
        xsess2.free_pixmap(drawable2);

        xsess.close();
        /* closing xsess2 breaks? */
        //xsess2.close();
    }
}

fn set_background<T: AsRef<str>>(path: T, mode: render::Mode) {

    let opt = BackgroundOptions::new(path, mode);
    opt.set_background();
}

fn tile_drawable(xsess: &xorg::XorgSession, d1: c_ulong, d2: c_ulong) {

    let mut gcvalues = xlib::XGCValues::initialize();
    let scr = xsess.screen();

    gcvalues.fill_style = xlib::FillTiled;
    gcvalues.tile = d1;

    let gc = xsess.gc(d2,
        (xlib::GCFillStyle | xlib::GCTile) as u64,
        &mut gcvalues
    );

    xsess.fill_rectangle(
        d2, gc, 0, 0,
        scr.width as u32, scr.height as u32
    );

    xsess.free_gc(gc);
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
    set_background(
        "/home/spowell/pictures/backgrounds/strangelove_1.png",
        render::Mode::Max
    );
}
