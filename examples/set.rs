extern crate xwallpaper;
use xwallpaper::render::Mode;

fn main() {
    xwallpaper::set_wallpaper(
        "/home/spowell/pictures/backgrounds/darkstar_poster.jpg",
        Mode::Center,
    ).expect("failed to set wallpaper");
}
