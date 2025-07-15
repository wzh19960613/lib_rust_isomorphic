/// Impl `AsRef` trait and then you can use `as_ref` method to get a `&TO` from a `&FROM`.
///
/// # Example
///
/// ```rust
/// use isomorphic::impl_ref;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// impl_ref!(ST<i32> = i32);
///
/// let v = 42;
/// let v_struct = ST(v);
/// let ref_struct_as_v: &i32 = v_struct.as_ref();  // OK
/// let ref_v_as_struct: &ST<i32> = v.as_ref();     // OK
/// assert_eq!(ref_struct_as_v, &ref_v_as_struct.0);
/// ```
/// # Usage
/// - `impl_ref!(FROM => TO)` will expand to `impl AsRef<TO> for FROM`
/// - `impl_ref!(AsRef<TO> for FROM)` will expand to `impl AsRef<TO> for FROM`
/// - `impl_ref!(T1 = T2)`
///   will expand to `impl AsRef<T1> for T2` and `impl AsRef<T2> for T1`
/// - Can also use generic parameters. For example:
///   `impl_ref!(FROM => TO where GENERIC)` will expand to
///   `impl<GENERIC> AsRef<TO> for FROM`
#[macro_export]
macro_rules! impl_ref {
    ($t_from: ty => $t_to: ty $(where $($g: tt)+)?) => {
        impl$(<$($g)+>)? std::convert::AsRef<$t_to> for $t_from {
            fn as_ref(&self) -> &$t_to {
                unsafe { $crate::ref_of(self) }
            }
        }
    };
    ($t_1: ty = $t_2: ty $(where $($g: tt)+)?) => {
        $crate::impl_ref!($t_1 => $t_2 $(where $($g)+)?);
        $crate::impl_ref!($t_2 => $t_1 $(where $($g)+)?);
    };
    (AsRef<$t_to: ty> for $t_from: ty $(where $($g: tt)+)?) => {
        $crate::impl_ref!($t_from => $t_to $(where $($g)+)?);
    }
}

/// Impl `AsMut` trait and then you can use `as_mut` method to get a `&mut TO` from a `&mut FROM`.
///
/// # Example
///
/// ```rust
/// use isomorphic::impl_mut;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// impl_mut!(ST<i32> = i32);
///
/// let mut v: i32 = 42;
/// let mut v_struct = ST(v);
/// let ref_mut_struct_as_v: &mut i32 = v_struct.as_mut();  // OK
/// let ref_v_as_mut_struct: &mut ST<i32> = v.as_mut();     // OK
/// *ref_mut_struct_as_v += 1;
/// ref_v_as_mut_struct.0 += 1;
/// assert_eq!(ref_mut_struct_as_v, &mut ref_v_as_mut_struct.0);
/// ```
/// # Usage
/// - `impl_mut!(FROM => TO)` will expand to `impl AsMut<TO> for FROM`
/// - `impl_mut!(AsMut<TO> for FROM)` will expand to `impl AsMut<TO> for FROM`
/// - `impl_mut!(T1 = T2)` will expand to `impl AsMut<T1> for T2` and `impl AsMut<T2> for T1`
/// - Can also use generic parameters. For example:
///   `impl_mut!(FROM => TO where GENERIC)` will expand to
///   `impl<GENERIC> AsMut<TO> for FROM`
#[macro_export]
macro_rules! impl_mut {
    ($t_from: ty => $t_to: ty $(where $($g: tt)+)?) => {
        impl$(<$($g)+>)? std::convert::AsMut<$t_to> for $t_from {
            fn as_mut(&mut self) -> &mut $t_to {
                unsafe { $crate::mut_ref_of(self) }
            }
        }
    };
    ($t_from: ty = $t_to: ty $(where $($g: tt)+)?) => {
        $crate::impl_mut!($t_from => $t_to $(where $($g)+)?);
        $crate::impl_mut!($t_to => $t_from $(where $($g)+)?);
    };
    (AsMut<$t_to: ty> for $t_from: ty $(where $($g: tt)+)?) => {
        $crate::impl_mut!($t_from => $t_to $(where $($g)+)?);
    }
}

/// Impl `From` trait and then you can use `from` and `into` methods to convert values.
///
/// # Example
///
/// ```rust
/// use isomorphic::impl_transmute;
///
/// #[repr(transparent)]
/// struct ST<T>(T);
///
/// impl_transmute!(ST<i32> = i32);
///
/// let v = 42;
/// let v_struct = ST(v);
/// let v_from_struct: i32 = v_struct.into();  // OK
/// let struct_from_v: ST<i32> = v.into();     // OK
/// assert_eq!(v_from_struct, v);
/// assert_eq!(struct_from_v.0, v);
/// ```
/// # Usage
/// - `impl_transmute!(FROM => TO)` will expand to `impl From<FROM> for TO`
/// - `impl_transmute!(From<FROM> for TO)` will expand to `impl From<FROM> for TO`
/// - `impl_transmute!(T1 = T2)` will expand to `impl From<T1> for T2` and `impl From<T2> for T1`
/// - Can also use generic parameters. For example:
///   `impl_transmute!(FROM => TO where GENERIC)` will expand to
///   `impl<GENERIC> From<FROM> for TO`
#[macro_export]
macro_rules! impl_transmute {
    ($t_from: ty => $t_to: ty $(where $($g: tt)+)?) => {
        impl$(<$($g)+>)? std::convert::From<$t_from> for $t_to {
            fn from(t: $t_from) -> Self {
                unsafe { $crate::clone_of(&t) }
            }
        }
    };
    ($t_from: ty = $t_to: ty $(where $($g: tt)+)?) => {
        $crate::impl_transmute!($t_from => $t_to $(where $($g)+)?);
        $crate::impl_transmute!($t_to => $t_from $(where $($g)+)?);
    };
    (From<$t_from: ty> for $t_to: ty $(where $($g: tt)+)?) => {
        $crate::impl_transmute!($t_from => $t_to $(where $($g)+)?);
    }
}
