// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Runtime type information.

use crate::translate::*;
use crate::Slice;

use std::fmt;
use std::mem;
use std::ptr;

// rustdoc-stripper-ignore-next
/// A GLib or GLib-based library type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(alias = "GType")]
#[repr(transparent)]
pub struct Type(ffi::GType);

impl Type {
    // rustdoc-stripper-ignore-next
    /// An invalid `Type` used as error return value in some functions
    #[doc(alias = "G_TYPE_INVALID")]
    pub const INVALID: Self = Self(gobject_ffi::G_TYPE_INVALID);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to the unit type `()`
    #[doc(alias = "G_TYPE_NONE")]
    pub const UNIT: Self = Self(gobject_ffi::G_TYPE_NONE);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `i8`
    #[doc(alias = "G_TYPE_CHAR")]
    pub const I8: Self = Self(gobject_ffi::G_TYPE_CHAR);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `u8`
    #[doc(alias = "G_TYPE_UCHAR")]
    pub const U8: Self = Self(gobject_ffi::G_TYPE_UCHAR);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `bool`
    #[doc(alias = "G_TYPE_BOOLEAN")]
    pub const BOOL: Self = Self(gobject_ffi::G_TYPE_BOOLEAN);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `i32`
    #[doc(alias = "G_TYPE_INT")]
    pub const I32: Self = Self(gobject_ffi::G_TYPE_INT);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `u32`
    #[doc(alias = "G_TYPE_UINT")]
    pub const U32: Self = Self(gobject_ffi::G_TYPE_UINT);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to C `long`
    #[doc(alias = "G_TYPE_LONG")]
    pub const I_LONG: Self = Self(gobject_ffi::G_TYPE_LONG);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to C `unsigned long`
    #[doc(alias = "G_TYPE_ULONG")]
    pub const U_LONG: Self = Self(gobject_ffi::G_TYPE_ULONG);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `i64`
    #[doc(alias = "G_TYPE_INT64")]
    pub const I64: Self = Self(gobject_ffi::G_TYPE_INT64);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `u64`
    #[doc(alias = "G_TYPE_UINT64")]
    pub const U64: Self = Self(gobject_ffi::G_TYPE_UINT64);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `f32`
    #[doc(alias = "G_TYPE_FLOAT")]
    pub const F32: Self = Self(gobject_ffi::G_TYPE_FLOAT);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `f64`
    #[doc(alias = "G_TYPE_DOUBLE")]
    pub const F64: Self = Self(gobject_ffi::G_TYPE_DOUBLE);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to `String`
    #[doc(alias = "G_TYPE_STRING")]
    pub const STRING: Self = Self(gobject_ffi::G_TYPE_STRING);

    // rustdoc-stripper-ignore-next
    /// The fundamental type corresponding to a pointer
    #[doc(alias = "G_TYPE_POINTER")]
    pub const POINTER: Self = Self(gobject_ffi::G_TYPE_POINTER);

    // rustdoc-stripper-ignore-next
    /// The fundamental type of GVariant
    #[doc(alias = "G_TYPE_VARIANT")]
    pub const VARIANT: Self = Self(gobject_ffi::G_TYPE_VARIANT);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all interfaces are derived
    #[doc(alias = "G_TYPE_INTERFACE")]
    pub const INTERFACE: Self = Self(gobject_ffi::G_TYPE_INTERFACE);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all enumeration types are derived
    #[doc(alias = "G_TYPE_ENUM")]
    pub const ENUM: Self = Self(gobject_ffi::G_TYPE_ENUM);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all flags types are derived
    #[doc(alias = "G_TYPE_FLAGS")]
    pub const FLAGS: Self = Self(gobject_ffi::G_TYPE_FLAGS);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all boxed types are derived
    #[doc(alias = "G_TYPE_BOXED")]
    pub const BOXED: Self = Self(gobject_ffi::G_TYPE_BOXED);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all `GParamSpec` types are derived
    #[doc(alias = "G_TYPE_PARAM")]
    pub const PARAM_SPEC: Self = Self(gobject_ffi::G_TYPE_PARAM);

    // rustdoc-stripper-ignore-next
    /// The fundamental type from which all objects are derived
    #[doc(alias = "G_TYPE_OBJECT")]
    pub const OBJECT: Self = Self(gobject_ffi::G_TYPE_OBJECT);

    #[doc(alias = "g_type_name")]
    pub fn name<'a>(self) -> &'a str {
        match self.into_glib() {
            gobject_ffi::G_TYPE_INVALID => "<invalid>",
            x => unsafe {
                let ptr = gobject_ffi::g_type_name(x);
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
            },
        }
    }

    #[doc(alias = "g_type_qname")]
    pub fn qname(self) -> crate::Quark {
        match self.into_glib() {
            gobject_ffi::G_TYPE_INVALID => crate::Quark::from_string("<invalid>"),
            x => unsafe { from_glib(gobject_ffi::g_type_qname(x)) },
        }
    }

    #[doc(alias = "g_type_is_a")]
    pub fn is_a(self, other: Self) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_type_is_a(
                self.into_glib(),
                other.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_type_parent")]
    pub fn parent(self) -> Option<Self> {
        unsafe {
            let parent: Self = from_glib(gobject_ffi::g_type_parent(self.into_glib()));
            Some(parent).filter(|t| t.is_valid())
        }
    }

    #[doc(alias = "g_type_children")]
    pub fn children(self) -> Slice<Self> {
        unsafe {
            let mut n_children = 0u32;
            let children = gobject_ffi::g_type_children(self.into_glib(), &mut n_children);

            Slice::from_glib_full_num_copy(children as *mut Self, n_children as usize)
        }
    }

    #[doc(alias = "g_type_interfaces")]
    pub fn interfaces(self) -> Slice<Self> {
        unsafe {
            let mut n_interfaces = 0u32;
            let interfaces = gobject_ffi::g_type_interfaces(self.into_glib(), &mut n_interfaces);

            Slice::from_glib_full_num_copy(interfaces as *mut Self, n_interfaces as usize)
        }
    }

    #[doc(alias = "g_type_interface_prerequisites")]
    pub fn interface_prerequisites(self) -> Slice<Self> {
        unsafe {
            match self {
                t if !t.is_a(Self::INTERFACE) => Slice::from_glib_full_num_copy(ptr::null_mut(), 0),
                _ => {
                    let mut n_prereqs = 0u32;
                    let prereqs = gobject_ffi::g_type_interface_prerequisites(
                        self.into_glib(),
                        &mut n_prereqs,
                    );

                    Slice::from_glib_full_num_copy(prereqs as *mut Self, n_prereqs as usize)
                }
            }
        }
    }

    #[doc(alias = "g_type_from_name")]
    pub fn from_name(name: &str) -> Option<Self> {
        unsafe {
            let type_: Self = from_glib(gobject_ffi::g_type_from_name(name.to_glib_none().0));
            Some(type_).filter(|t| t.is_valid())
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks that the type is not [`INVALID`](Self::INVALID)
    #[inline]
    pub fn is_valid(self) -> bool {
        self != Self::INVALID
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

// rustdoc-stripper-ignore-next
/// Types that are supported by GLib dynamic typing.
pub trait StaticType {
    // rustdoc-stripper-ignore-next
    /// Returns the type identifier of `Self`.
    fn static_type() -> Type;
}

impl StaticType for Type {
    #[doc(alias = "g_gtype_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_gtype_get_type()) }
    }
}

#[doc(hidden)]
impl crate::value::ValueType for Type {
    type Type = Type;
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for Type {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        from_glib(gobject_ffi::g_value_get_gtype(value.to_glib_none().0))
    }
}

#[doc(hidden)]
impl crate::value::ToValue for Type {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type(Type::static_type());
            gobject_ffi::g_value_set_gtype(value.to_glib_none_mut().0, self.into_glib());
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        Type::static_type()
    }
}

impl<'a, T: ?Sized + StaticType> StaticType for &'a T {
    fn static_type() -> Type {
        T::static_type()
    }
}

impl<'a, T: ?Sized + StaticType> StaticType for &'a mut T {
    fn static_type() -> Type {
        T::static_type()
    }
}

macro_rules! builtin {
    ($name:ty, $val:ident) => {
        impl StaticType for $name {
            fn static_type() -> Type {
                Type::$val
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ILong(pub libc::c_long);

impl std::ops::Deref for ILong {
    type Target = libc::c_long;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ILong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<libc::c_long> for ILong {
    fn from(v: libc::c_long) -> ILong {
        ILong(v)
    }
}

impl From<ILong> for libc::c_long {
    fn from(v: ILong) -> libc::c_long {
        v.0
    }
}

impl PartialEq<libc::c_long> for ILong {
    fn eq(&self, other: &libc::c_long) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ILong> for libc::c_long {
    fn eq(&self, other: &ILong) -> bool {
        self == &other.0
    }
}

impl PartialOrd<libc::c_long> for ILong {
    fn partial_cmp(&self, other: &libc::c_long) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ILong> for libc::c_long {
    fn partial_cmp(&self, other: &ILong) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ULong(pub libc::c_ulong);

impl std::ops::Deref for ULong {
    type Target = libc::c_ulong;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ULong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<libc::c_ulong> for ULong {
    fn from(v: libc::c_ulong) -> ULong {
        ULong(v)
    }
}

impl From<ULong> for libc::c_ulong {
    fn from(v: ULong) -> libc::c_ulong {
        v.0
    }
}

impl PartialEq<libc::c_ulong> for ULong {
    fn eq(&self, other: &libc::c_ulong) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ULong> for libc::c_ulong {
    fn eq(&self, other: &ULong) -> bool {
        self == &other.0
    }
}

impl PartialOrd<libc::c_ulong> for ULong {
    fn partial_cmp(&self, other: &libc::c_ulong) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ULong> for libc::c_ulong {
    fn partial_cmp(&self, other: &ULong) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

builtin!(bool, BOOL);
builtin!(i8, I8);
builtin!(u8, U8);
builtin!(i32, I32);
builtin!(u32, U32);
builtin!(i64, I64);
builtin!(u64, U64);
builtin!(ILong, I_LONG);
builtin!(ULong, U_LONG);
builtin!(f32, F32);
builtin!(f64, F64);
builtin!(str, STRING);
builtin!(String, STRING);

impl<'a> StaticType for [&'a str] {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_strv_get_type()) }
    }
}

impl StaticType for Vec<String> {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_strv_get_type()) }
    }
}

impl StaticType for () {
    fn static_type() -> Type {
        Type::UNIT
    }
}

#[inline]
pub unsafe fn instance_of<C: StaticType>(ptr: ffi::gconstpointer) -> bool {
    from_glib(gobject_ffi::g_type_check_instance_is_a(
        ptr as *mut _,
        <C as StaticType>::static_type().into_glib(),
    ))
}

impl FromGlib<ffi::GType> for Type {
    #[inline]
    unsafe fn from_glib(val: ffi::GType) -> Self {
        Self(val)
    }
}

impl IntoGlib for Type {
    type GlibType = ffi::GType;

    #[inline]
    fn into_glib(self) -> ffi::GType {
        self.0
    }
}

impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::GType> for Type {
    type Storage = Option<Vec<ffi::GType>>;

    fn to_glib_none_from_slice(t: &'a [Type]) -> (*mut ffi::GType, Self::Storage) {
        let mut vec = t.iter().map(|t| t.into_glib()).collect::<Vec<_>>();

        (vec.as_mut_ptr(), Some(vec))
    }

    fn to_glib_container_from_slice(t: &'a [Type]) -> (*mut ffi::GType, Self::Storage) {
        (Self::to_glib_full_from_slice(t), None)
    }

    fn to_glib_full_from_slice(t: &[Type]) -> *mut ffi::GType {
        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res =
                ffi::g_malloc0(mem::size_of::<ffi::GType>() * (t.len() + 1)) as *mut ffi::GType;
            for (i, v) in t.iter().enumerate() {
                *res.add(i) = v.into_glib();
            }
            res
        }
    }
}

impl FromGlibContainerAsVec<Type, *const ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

impl FromGlibContainerAsVec<Type, *mut ffi::GType> for Type {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InitiallyUnowned;
    use std::collections::{BTreeSet, HashSet};

    #[test]
    fn invalid() {
        let invalid = Type::INVALID;

        assert_eq!(invalid.name(), "<invalid>");
        assert_eq!(invalid.qname(), crate::Quark::from_string("<invalid>"));
        assert!(invalid.is_a(Type::INVALID));
        assert!(!invalid.is_a(Type::STRING));
        assert_eq!(invalid.parent(), None);
        assert!(invalid.children().is_empty());
        assert!(invalid.interfaces().is_empty());
        assert!(invalid.interface_prerequisites().is_empty());
        assert!(!invalid.is_valid());
        dbg!(&invalid);
    }

    #[test]
    fn hash() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();

        let set = Type::OBJECT
            .children()
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        assert!(set.contains(&iu_type));
    }

    #[test]
    fn ord() {
        // Get this first so the type is registered
        let iu_type = InitiallyUnowned::static_type();
        assert!(Type::OBJECT < iu_type);

        let set = Type::OBJECT
            .children()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert!(set.contains(&iu_type));
    }
}
