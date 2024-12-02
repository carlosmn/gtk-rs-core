// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, SocketAddress, SocketConnectable};

glib::wrapper! {
    #[doc(alias = "GNativeSocketAddress")]
    pub struct NativeSocketAddress(Object<ffi::GNativeSocketAddress, ffi::GNativeSocketAddressClass>) @extends SocketAddress, @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_native_socket_address_get_type(),
    }
}

impl NativeSocketAddress {
    pub const NONE: Option<&'static NativeSocketAddress> = None;

    //#[doc(alias = "g_native_socket_address_new")]
    //pub fn new(native: /*Unimplemented*/Option<Basic: Pointer>, len: usize) -> NativeSocketAddress {
    //    unsafe { TODO: call ffi:g_native_socket_address_new() }
    //}
}