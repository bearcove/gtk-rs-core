// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Box;
use crate::Point3D;
use glib::translate::*;

glib::wrapper! {
    pub struct Sphere(BoxedInline<ffi::graphene_sphere_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_sphere_get_type(), ptr as *mut _) as *mut ffi::graphene_sphere_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_sphere_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_sphere_get_type(),
    }
}

impl Sphere {
    #[doc(alias = "graphene_sphere_contains_point")]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        unsafe {
            ffi::graphene_sphere_contains_point(self.to_glib_none().0, point.to_glib_none().0)
        }
    }

    #[doc(alias = "graphene_sphere_distance")]
    pub fn distance(&self, point: &Point3D) -> f32 {
        unsafe { ffi::graphene_sphere_distance(self.to_glib_none().0, point.to_glib_none().0) }
    }

    #[doc(alias = "graphene_sphere_equal")]
    fn equal(&self, b: &Sphere) -> bool {
        unsafe { ffi::graphene_sphere_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_sphere_get_bounding_box")]
    #[doc(alias = "get_bounding_box")]
    pub fn bounding_box(&self) -> Box {
        unsafe {
            let mut box_ = Box::uninitialized();
            ffi::graphene_sphere_get_bounding_box(self.to_glib_none().0, box_.to_glib_none_mut().0);
            box_
        }
    }

    #[doc(alias = "graphene_sphere_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self) -> Point3D {
        unsafe {
            let mut center = Point3D::uninitialized();
            ffi::graphene_sphere_get_center(self.to_glib_none().0, center.to_glib_none_mut().0);
            center
        }
    }

    #[doc(alias = "graphene_sphere_get_radius")]
    #[doc(alias = "get_radius")]
    pub fn radius(&self) -> f32 {
        unsafe { ffi::graphene_sphere_get_radius(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_sphere_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::graphene_sphere_is_empty(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_sphere_translate")]
    pub fn translate(&self, point: &Point3D) -> Sphere {
        unsafe {
            let mut res = Sphere::uninitialized();
            ffi::graphene_sphere_translate(
                self.to_glib_none().0,
                point.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }
}

impl PartialEq for Sphere {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Sphere {}
