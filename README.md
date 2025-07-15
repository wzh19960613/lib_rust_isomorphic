# isomorphic

A crate for getting a reference to an isomorphic type.

## Usage

### `ref_of`, `mut_ref_of` and `clone_of`

```rust
use isomorphic::{ref_of, mut_ref_of, clone_of};

#[repr(transparent)]
struct ST<T>(T);

#[repr(transparent)]
enum ET<T> {
    Foo(T),
}

let v = 42;
let v_struct = ST(v);
let ref_struct_as_v: &i32 = unsafe { ref_of(&v_struct) }; // OK
assert_eq!(ref_struct_as_v, &v);
assert_eq!(v_struct.0, v);

let v_enum = ET::Foo(42);
let ref_enum_as_v: &i32 = unsafe { ref_of(&v_enum) }; // OK
assert_eq!(ref_enum_as_v, &v);
assert_eq!(v, unsafe { clone_of(&v_enum) });

let v_next = v + 1;

let mut v_mut_struct = ST(v);
let ref_mut_struct_as_v: &mut i32 = unsafe { mut_ref_of(&mut v_mut_struct) }; // OK
assert_eq!(ref_mut_struct_as_v, &v);
*ref_mut_struct_as_v = v_next;
assert_eq!(v_mut_struct.0, v_next);

let mut v_mut_enum = ET::Foo(42);
let ref_mut_enum_as_v: &mut i32 = unsafe { mut_ref_of(&mut v_mut_enum) }; // OK
assert_eq!(ref_mut_enum_as_v, &v);
*ref_mut_enum_as_v = v_next;
assert_eq!(v_next, unsafe { clone_of(&v_mut_enum) });
```

### Macros: `impl_ref`, `impl_mut` and `impl_transmute`

When the feature `macros` is enabled, you can use the macros `impl_ref`, `impl_mut` and `impl_transmute` to automatically implement the `AsRef`, `AsMut` and `From` traits for your types.

```rust, cfg(feature = "macros")
// With `macros` feature for `isomorphic`

use isomorphic::{impl_ref, impl_mut, impl_transmute};

#[repr(transparent)]
struct ST<T>(T);

impl_ref!(ST<i32> = i32);

let v = 42;
let v_struct = ST(v);
let ref_struct_as_v: &i32 = v_struct.as_ref();  // OK
let ref_v_as_struct: &ST<i32> = v.as_ref();     // OK
assert_eq!(ref_struct_as_v, &ref_v_as_struct.0);

impl_mut!(ST<i32> = i32);

let mut v = 42;
let mut v_struct = ST(v);
let ref_mut_struct_as_v: &mut i32 = v_struct.as_mut();  // OK
let ref_v_as_mut_struct: &mut ST<i32> = v.as_mut();     // OK
*ref_mut_struct_as_v += 1;
ref_v_as_mut_struct.0 += 1;
assert_eq!(ref_mut_struct_as_v, &mut ref_v_as_mut_struct.0);

impl_transmute!(ST<i32> = i32);

let v = 42;
let v_struct = ST(v);
let v_from_struct: i32 = v_struct.into();  // OK
let struct_from_v: ST<i32> = v.into();     // OK
assert_eq!(v_from_struct, v);
assert_eq!(struct_from_v.0, v);
```