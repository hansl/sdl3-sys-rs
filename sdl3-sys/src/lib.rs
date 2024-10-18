#![no_std]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(c_variadic))] // https://github.com/rust-lang/rust/issues/44930
#![cfg_attr(feature = "nightly", feature(stdarch_arm_hints))] // https://github.com/rust-lang/rust/issues/117218
#![doc = include_str!("../README.md")]

use core::mem::size_of;

// This macro is used to apply cfg attributes to a group of items. Wrap the items
// in a call to this macro and apply the attributes to the macro call
macro_rules! emit {
    ($($tt:tt)*) => { $($tt)* };
}

// Get the size of a field of a struct or union
macro_rules! size_of_field {
    ($struct:ty, $field:ident) => {
        $crate::size_of_return_value(&|s: $struct| unsafe {
            // safety: this is never evaluated
            s.$field
        })
    };
}
pub(crate) use size_of_field;

#[allow(unused)] // incorrectly detected as unused
const fn size_of_return_value<T, R>(_: &impl FnOnce(T) -> R) -> usize {
    size_of::<R>()
}

#[doc(hidden)] // for internal use only
#[macro_export]
macro_rules! __const_c_str {
    ($cstr:ident = $str:expr) => {
        const $cstr: [::core::ffi::c_char; $str.len() + 1] = {
            const BYTES: &[::core::primitive::u8] = $str.as_bytes();
            let mut cstr = [0 as ::core::ffi::c_char; BYTES.len() + 1];
            let mut i = 0;
            while i < BYTES.len() {
                assert!(BYTES[i] != 0, "zero byte in string");
                cstr[i] = BYTES[i] as ::core::ffi::c_char;
                i += 1;
            }
            cstr
        };
    };
}

mod generated;
pub use generated::*;

/// Extra ffi types for `sdl3-sys`
pub mod ffi {
    #[cfg(doc)]
    /// Equivalent to C's `wchar_t` type. This is `u16` on Windows and `u32` otherwise.
    /// Enable the `use-libc` feature to make this an alias of `libc::wchar_t`.
    pub type c_wchar_t = u32;
    #[cfg(all(not(doc), feature = "use-libc"))]
    pub type c_wchar_t = ::libc::wchar_t;
    #[cfg(all(not(any(doc, feature = "use-libc")), windows))]
    pub type c_wchar_t = u16;
    #[cfg(all(not(any(doc, feature = "use-libc")), not(windows)))]
    pub type c_wchar_t = u32;

    #[cfg(doc)]
    /// Equivalent to C's `va_list` type. Enable the `nightly` feature and compile with
    /// the nightly compiler to make this an alias of [`core::ffi::VaList`]. Otherwise,
    /// this type can't be instantiated.
    pub enum VaList {}
    #[cfg(all(not(doc), feature = "nightly"))]
    pub use core::ffi::VaList;
    #[cfg(all(not(doc), not(feature = "nightly")))]
    pub enum VaList {}
}
