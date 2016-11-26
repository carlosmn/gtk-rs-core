// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureZoom(Object<ffi::GtkGestureZoom>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureZoom {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_zoom_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_scale_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_scale_changed<F: Fn(&GestureZoom, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureZoom, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scale-changed",
                transmute(scale_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn scale_changed_trampoline(this: *mut ffi::GtkGestureZoom, scale: libc::c_double, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureZoom, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), scale)
}
