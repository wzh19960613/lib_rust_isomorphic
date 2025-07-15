#![doc = include_str!("../README.md")]

#[cfg(feature = "macros")]
#[macro_use]
mod macros;

/// # Example
///
/// ```rust
/// use isomorphic::ref_of;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// let v = 42;
/// let v_struct = ST(v);
/// let ref_struct_as_v: &i32 = unsafe { ref_of(&v_struct) }; // OK
/// assert_eq!(ref_struct_as_v, &v);
/// ```
///
/// # Safety
///
/// Make sure that `FROM` and `TO` are isomorphic.
pub unsafe fn ref_of<SRC, DST>(from: &SRC) -> &DST {
    unsafe { &*(from as *const SRC as *const DST) }
}

/// # Example
///
/// ```rust
/// use isomorphic::mut_ref_of;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// let v = 42;
/// let mut v_struct = ST(v);
/// let ref_struct_as_v: &mut i32 = unsafe { mut_ref_of(&mut v_struct) }; // OK
/// *ref_struct_as_v += 1;
/// assert_eq!(v_struct.0, v + 1);
/// ```
///
/// # Safety
///
/// Make sure that `FROM` and `TO` are isomorphic.
pub unsafe fn mut_ref_of<SRC, DST>(from: &mut SRC) -> &mut DST {
    unsafe { &mut *(from as *mut SRC as *mut DST) }
}

/// This is just a wrapper of `std::mem::transmute_copy`.
///
/// # Example
///
/// ```rust
/// use isomorphic::clone_of;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// let v = 42;
/// let v_struct = ST(v);
/// let clone_from_struct: i32 = unsafe { clone_of(&v_struct) };
/// assert_eq!(clone_from_struct, v);
/// ```
///
pub use std::mem::transmute_copy as clone_of;
