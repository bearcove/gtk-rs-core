// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Box;
use crate::Euler;
use crate::Point;
use crate::Point3D;
use crate::Quad;
use crate::Quaternion;
use crate::Ray;
use crate::Rect;
use crate::Sphere;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    pub struct Matrix(BoxedInline<ffi::graphene_matrix_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_matrix_get_type(), ptr as *mut _) as *mut ffi::graphene_matrix_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_matrix_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_matrix_get_type(),
    }
}

impl Matrix {
    #[doc(alias = "graphene_matrix_decompose")]
    pub fn decompose(&self) -> Option<(Vec3, Vec3, Quaternion, Vec3, Vec4)> {
        unsafe {
            let mut translate = Vec3::uninitialized();
            let mut scale = Vec3::uninitialized();
            let mut rotate = Quaternion::uninitialized();
            let mut shear = Vec3::uninitialized();
            let mut perspective = Vec4::uninitialized();
            let ret = ffi::graphene_matrix_decompose(
                self.to_glib_none().0,
                translate.to_glib_none_mut().0,
                scale.to_glib_none_mut().0,
                rotate.to_glib_none_mut().0,
                shear.to_glib_none_mut().0,
                perspective.to_glib_none_mut().0,
            );
            if ret {
                Some((translate, scale, rotate, shear, perspective))
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_matrix_determinant")]
    pub fn determinant(&self) -> f32 {
        unsafe { ffi::graphene_matrix_determinant(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_equal")]
    fn equal(&self, b: &Matrix) -> bool {
        unsafe { ffi::graphene_matrix_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_equal_fast")]
    pub fn equal_fast(&self, b: &Matrix) -> bool {
        unsafe { ffi::graphene_matrix_equal_fast(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_row")]
    #[doc(alias = "get_row")]
    pub fn row(&self, index_: u32) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_matrix_get_row(self.to_glib_none().0, index_, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_matrix_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, row: u32, col: u32) -> f32 {
        unsafe { ffi::graphene_matrix_get_value(self.to_glib_none().0, row, col) }
    }

    #[doc(alias = "graphene_matrix_get_x_scale")]
    #[doc(alias = "get_x_scale")]
    pub fn x_scale(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_x_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_x_translation")]
    #[doc(alias = "get_x_translation")]
    pub fn x_translation(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_x_translation(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_y_scale")]
    #[doc(alias = "get_y_scale")]
    pub fn y_scale(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_y_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_y_translation")]
    #[doc(alias = "get_y_translation")]
    pub fn y_translation(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_y_translation(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_z_scale")]
    #[doc(alias = "get_z_scale")]
    pub fn z_scale(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_z_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_get_z_translation")]
    #[doc(alias = "get_z_translation")]
    pub fn z_translation(&self) -> f32 {
        unsafe { ffi::graphene_matrix_get_z_translation(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_interpolate")]
    pub fn interpolate(&self, b: &Matrix, factor: f64) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_interpolate(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_inverse")]
    pub fn inverse(&self) -> Option<Matrix> {
        unsafe {
            let mut res = Matrix::uninitialized();
            let ret = ffi::graphene_matrix_inverse(self.to_glib_none().0, res.to_glib_none_mut().0);
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_matrix_is_2d")]
    pub fn is_2d(&self) -> bool {
        unsafe { ffi::graphene_matrix_is_2d(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_is_backface_visible")]
    pub fn is_backface_visible(&self) -> bool {
        unsafe { ffi::graphene_matrix_is_backface_visible(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_is_identity")]
    pub fn is_identity(&self) -> bool {
        unsafe { ffi::graphene_matrix_is_identity(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_is_singular")]
    pub fn is_singular(&self) -> bool {
        unsafe { ffi::graphene_matrix_is_singular(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_matrix_multiply")]
    pub fn multiply(&self, b: &Matrix) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_multiply(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_near")]
    pub fn near(&self, b: &Matrix, epsilon: f32) -> bool {
        unsafe { ffi::graphene_matrix_near(self.to_glib_none().0, b.to_glib_none().0, epsilon) }
    }

    #[doc(alias = "graphene_matrix_normalize")]
    pub fn normalize(&self) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_matrix_perspective")]
    pub fn perspective(&self, depth: f32) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_perspective(
                self.to_glib_none().0,
                depth,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_print")]
    pub fn print(&self) {
        unsafe {
            ffi::graphene_matrix_print(self.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_matrix_project_point")]
    pub fn project_point(&self, p: &Point) -> Point {
        unsafe {
            let mut res = Point::uninitialized();
            ffi::graphene_matrix_project_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_project_rect")]
    pub fn project_rect(&self, r: &Rect) -> Quad {
        unsafe {
            let mut res = Quad::uninitialized();
            ffi::graphene_matrix_project_rect(
                self.to_glib_none().0,
                r.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_project_rect_bounds")]
    pub fn project_rect_bounds(&self, r: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_project_rect_bounds(
                self.to_glib_none().0,
                r.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_rotate")]
    pub fn rotate(&mut self, angle: f32, axis: &Vec3) {
        unsafe {
            ffi::graphene_matrix_rotate(self.to_glib_none_mut().0, angle, axis.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_matrix_rotate_euler")]
    pub fn rotate_euler(&mut self, e: &Euler) {
        unsafe {
            ffi::graphene_matrix_rotate_euler(self.to_glib_none_mut().0, e.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_matrix_rotate_quaternion")]
    pub fn rotate_quaternion(&mut self, q: &Quaternion) {
        unsafe {
            ffi::graphene_matrix_rotate_quaternion(self.to_glib_none_mut().0, q.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_matrix_rotate_x")]
    pub fn rotate_x(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_x(self.to_glib_none_mut().0, angle);
        }
    }

    #[doc(alias = "graphene_matrix_rotate_y")]
    pub fn rotate_y(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_y(self.to_glib_none_mut().0, angle);
        }
    }

    #[doc(alias = "graphene_matrix_rotate_z")]
    pub fn rotate_z(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_z(self.to_glib_none_mut().0, angle);
        }
    }

    #[doc(alias = "graphene_matrix_scale")]
    pub fn scale(&mut self, factor_x: f32, factor_y: f32, factor_z: f32) {
        unsafe {
            ffi::graphene_matrix_scale(self.to_glib_none_mut().0, factor_x, factor_y, factor_z);
        }
    }

    #[doc(alias = "graphene_matrix_skew_xy")]
    pub fn skew_xy(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_xy(self.to_glib_none_mut().0, factor);
        }
    }

    #[doc(alias = "graphene_matrix_skew_xz")]
    pub fn skew_xz(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_xz(self.to_glib_none_mut().0, factor);
        }
    }

    #[doc(alias = "graphene_matrix_skew_yz")]
    pub fn skew_yz(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_yz(self.to_glib_none_mut().0, factor);
        }
    }

    #[doc(alias = "graphene_matrix_to_2d")]
    pub fn to_2d(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        unsafe {
            let mut xx = mem::MaybeUninit::uninit();
            let mut yx = mem::MaybeUninit::uninit();
            let mut xy = mem::MaybeUninit::uninit();
            let mut yy = mem::MaybeUninit::uninit();
            let mut x_0 = mem::MaybeUninit::uninit();
            let mut y_0 = mem::MaybeUninit::uninit();
            let ret = ffi::graphene_matrix_to_2d(
                self.to_glib_none().0,
                xx.as_mut_ptr(),
                yx.as_mut_ptr(),
                xy.as_mut_ptr(),
                yy.as_mut_ptr(),
                x_0.as_mut_ptr(),
                y_0.as_mut_ptr(),
            );
            let xx = xx.assume_init();
            let yx = yx.assume_init();
            let xy = xy.assume_init();
            let yy = yy.assume_init();
            let x_0 = x_0.assume_init();
            let y_0 = y_0.assume_init();
            if ret {
                Some((xx, yx, xy, yy, x_0, y_0))
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_matrix_transform_bounds")]
    pub fn transform_bounds(&self, r: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_transform_bounds(
                self.to_glib_none().0,
                r.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_box")]
    pub fn transform_box(&self, b: &Box) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_matrix_transform_box(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_point")]
    pub fn transform_point(&self, p: &Point) -> Point {
        unsafe {
            let mut res = Point::uninitialized();
            ffi::graphene_matrix_transform_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_point3d")]
    pub fn transform_point3d(&self, p: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_matrix_transform_point3d(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_ray")]
    pub fn transform_ray(&self, r: &Ray) -> Ray {
        unsafe {
            let mut res = Ray::uninitialized();
            ffi::graphene_matrix_transform_ray(
                self.to_glib_none().0,
                r.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_rect")]
    pub fn transform_rect(&self, r: &Rect) -> Quad {
        unsafe {
            let mut res = Quad::uninitialized();
            ffi::graphene_matrix_transform_rect(
                self.to_glib_none().0,
                r.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_sphere")]
    pub fn transform_sphere(&self, s: &Sphere) -> Sphere {
        unsafe {
            let mut res = Sphere::uninitialized();
            ffi::graphene_matrix_transform_sphere(
                self.to_glib_none().0,
                s.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_vec3")]
    pub fn transform_vec3(&self, v: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_matrix_transform_vec3(
                self.to_glib_none().0,
                v.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_transform_vec4")]
    pub fn transform_vec4(&self, v: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_matrix_transform_vec4(
                self.to_glib_none().0,
                v.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_translate")]
    pub fn translate(&mut self, pos: &Point3D) {
        unsafe {
            ffi::graphene_matrix_translate(self.to_glib_none_mut().0, pos.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_matrix_transpose")]
    pub fn transpose(&self) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_transpose(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_matrix_unproject_point3d")]
    pub fn unproject_point3d(&self, modelview: &Matrix, point: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_matrix_unproject_point3d(
                self.to_glib_none().0,
                modelview.to_glib_none().0,
                point.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_untransform_bounds")]
    pub fn untransform_bounds(&self, r: &Rect, bounds: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_untransform_bounds(
                self.to_glib_none().0,
                r.to_glib_none().0,
                bounds.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_matrix_untransform_point")]
    pub fn untransform_point(&self, p: &Point, bounds: &Rect) -> Option<Point> {
        unsafe {
            let mut res = Point::uninitialized();
            let ret = ffi::graphene_matrix_untransform_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
                bounds.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }
}

impl PartialEq for Matrix {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Matrix {}
