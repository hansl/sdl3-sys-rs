#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryPlatform
//!
//! SDL provides a means to identify the app's platform, both at compile time
//! and runtime.

extern_sdlcall! {{
    /// Get the name of the platform.
    ///
    /// Here are the names returned for some (but not all) supported platforms:
    ///
    /// - "Windows"
    /// - "macOS"
    /// - "Linux"
    /// - "iOS"
    /// - "Android"
    ///
    /// \returns the name of the platform. If the correct platform name is not
    ///          available, returns a string beginning with the text "Unknown".
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetPlatform() -> *const ::core::ffi::c_char;
}}
