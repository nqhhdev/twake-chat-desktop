fn main() {
    slint_build::compile("ui/app.slint").expect("Slint UI compiles");
    // libwebrtc (calls, ADR-018) keeps Objective-C categories in its static library:
    // without -ObjC the linker drops them and the call crashes on its first video codec
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg-bins=-ObjC");
    }
}
