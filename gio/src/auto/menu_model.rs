// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{MenuAttributeIter, MenuLinkIter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GMenuModel")]
    pub struct MenuModel(Object<ffi::GMenuModel, ffi::GMenuModelClass>);

    match fn {
        type_ => || ffi::g_menu_model_get_type(),
    }
}

impl MenuModel {
    pub const NONE: Option<&'static MenuModel> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::MenuModel>> Sealed for T {}
}

pub trait MenuModelExt: IsA<MenuModel> + sealed::Sealed + 'static {
    //#[doc(alias = "g_menu_model_get_item_attribute")]
    //#[doc(alias = "get_item_attribute")]
    //fn is_item_attribute(&self, item_index: i32, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:g_menu_model_get_item_attribute() }
    //}

    #[doc(alias = "g_menu_model_get_item_attribute_value")]
    #[doc(alias = "get_item_attribute_value")]
    fn item_attribute_value(
        &self,
        item_index: i32,
        attribute: &str,
        expected_type: Option<&glib::VariantTy>,
    ) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_attribute_value(
                self.as_ref().to_glib_none().0,
                item_index,
                attribute.to_glib_none().0,
                expected_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_menu_model_get_item_link")]
    #[doc(alias = "get_item_link")]
    #[must_use]
    fn item_link(&self, item_index: i32, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_link(
                self.as_ref().to_glib_none().0,
                item_index,
                link.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_menu_model_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> i32 {
        unsafe { ffi::g_menu_model_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_menu_model_is_mutable")]
    fn is_mutable(&self) -> bool {
        unsafe { from_glib(ffi::g_menu_model_is_mutable(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_menu_model_items_changed")]
    fn items_changed(&self, position: i32, removed: i32, added: i32) {
        unsafe {
            ffi::g_menu_model_items_changed(
                self.as_ref().to_glib_none().0,
                position,
                removed,
                added,
            );
        }
    }

    #[doc(alias = "g_menu_model_iterate_item_attributes")]
    fn iterate_item_attributes(&self, item_index: i32) -> MenuAttributeIter {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_attributes(
                self.as_ref().to_glib_none().0,
                item_index,
            ))
        }
    }

    #[doc(alias = "g_menu_model_iterate_item_links")]
    fn iterate_item_links(&self, item_index: i32) -> MenuLinkIter {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_links(
                self.as_ref().to_glib_none().0,
                item_index,
            ))
        }
    }

    #[doc(alias = "items-changed")]
    fn connect_items_changed<F: Fn(&Self, i32, i32, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn items_changed_trampoline<
            P: IsA<MenuModel>,
            F: Fn(&P, i32, i32, i32) + 'static,
        >(
            this: *mut ffi::GMenuModel,
            position: libc::c_int,
            removed: libc::c_int,
            added: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                MenuModel::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                removed,
                added,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"items-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    items_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<MenuModel>> MenuModelExt for O {}
