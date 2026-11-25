fn main() {
    // Linux: libwebrtc's screen capture goes through the desktop portal (GDBus, GVariant),
    // but webrtc-sys reads only the headers of GLib: the app links the libraries itself.
    // A dependency's libraries come after every Rust library on the link line, so they
    // resolve the symbols of libwebrtc.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        for lib in ["gio-2.0", "gobject-2.0", "glib-2.0"] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }
}
