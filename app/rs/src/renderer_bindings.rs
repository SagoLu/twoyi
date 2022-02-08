/* automatically generated by rust-bindgen 0.59.2 */

#[allow(dead_code)]
#[link(name="OpenglRender")]
extern "C" {
    pub fn destroyOpenGLSubwindow() -> ::std::os::raw::c_int;

    pub fn repaintOpenGLDisplay();

    pub fn setNativeWindow(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;

    pub fn resetSubWindow(
        p_window: *mut ::std::os::raw::c_void,
        wx: ::std::os::raw::c_int,
        wy: ::std::os::raw::c_int,
        ww: ::std::os::raw::c_int,
        wh: ::std::os::raw::c_int,
        fbw: ::std::os::raw::c_int,
        fbh: ::std::os::raw::c_int,
        dpr: f32,
        zRot: f32,
    ) -> ::std::os::raw::c_int;

    pub fn startOpenGLRenderer(
        win: *mut ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        xdpi: ::std::os::raw::c_int,
        ydpi: ::std::os::raw::c_int,
        fps: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn removeSubWindow(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
