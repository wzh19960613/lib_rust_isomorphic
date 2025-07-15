#![doc = include_str!("../README.md")]

#[cfg(feature = "macros")]
#[macro_use]
mod macros;

/// # Safety
///
/// Make sure that `FROM` and `TO` are isomorphic.
pub unsafe fn ref_of<SRC, DST>(from: &SRC) -> &DST {
    unsafe { &*(from as *const SRC as *const DST) }
}

/// # Safety
///
/// Make sure that `FROM` and `TO` are isomorphic.
pub unsafe fn mut_ref_of<SRC, DST>(from: &mut SRC) -> &mut DST {
    unsafe { &mut *(from as *mut SRC as *mut DST) }
}

/// This is just a wrapper of `std::mem::transmute_copy`.
pub use std::mem::transmute_copy as clone_of;
