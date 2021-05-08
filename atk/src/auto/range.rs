// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Range(Boxed<ffi::AtkRange>);

    match fn {
        copy => |ptr| ffi::atk_range_copy(mut_override(ptr)),
        free => |ptr| ffi::atk_range_free(ptr),
        type_ => || ffi::atk_range_get_type(),
    }
}

impl Range {
    #[doc(alias = "atk_range_new")]
    pub fn new(lower_limit: f64, upper_limit: f64, description: &str) -> Range {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::atk_range_new(
                lower_limit,
                upper_limit,
                description.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_range_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&mut self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::atk_range_get_description(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "atk_range_get_lower_limit")]
    #[doc(alias = "get_lower_limit")]
    pub fn lower_limit(&mut self) -> f64 {
        unsafe { ffi::atk_range_get_lower_limit(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "atk_range_get_upper_limit")]
    #[doc(alias = "get_upper_limit")]
    pub fn upper_limit(&mut self) -> f64 {
        unsafe { ffi::atk_range_get_upper_limit(self.to_glib_none_mut().0) }
    }
}
