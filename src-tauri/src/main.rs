// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Issue #34: WebKitGTK on a Wayland session used to fall back to Xwayland,
    // and Xwayland hands the webview the unscaled (1.0) monitor factor, so the
    // editor rendered at design-time font sizes on a 1.5x HiDPI composition.
    // GDK's backend list is "first that works": asking for wayland first means
    // a Wayland session runs natively there and picks up the compositor's
    // scale, while a headless or X11-only host falls through to x11 without
    // a crash. Only the default is set here; anything the user already pinned
    // in their launcher wins.
    //
    // The AppImage's bundled GTK still has the Wayland crash of
    // tauri-apps/tauri#8541, so the AppImage launcher keeps forcing X11 there.
    // osg-host-webkit.sh carries the same default for the host stack, where
    // the GTK is recent enough to be fine.
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WAYLAND_DISPLAY").is_some()
            && std::env::var_os("GDK_BACKEND").is_none()
        {
            std::env::set_var("GDK_BACKEND", "wayland,x11");
        }
    }

    open_screenshot_generator_lib::run();
}
