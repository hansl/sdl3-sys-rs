#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryVideo
//!
//! SDL video functions.

use super::stdinc::*;

use super::error::*;

use super::pixels::*;

use super::properties::*;

use super::rect::*;

use super::surface::*;

/// This is a unique ID for a display for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// If the display is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_DisplayID = Uint32;

/// This is a unique ID for a window.
///
/// The value 0 is an invalid ID.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_WindowID = Uint32;

pub const SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.video.wayland.wl_display\0") };

/// System theme.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_SYSTEM_THEME_UNKNOWN`], [`SDL_SYSTEM_THEME_LIGHT`], [`SDL_SYSTEM_THEME_DARK`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_SystemTheme(pub ::core::ffi::c_int);
impl SDL_SystemTheme {
    /// Unknown system theme
    pub const UNKNOWN: Self = Self(0);
    /// Light colored system theme
    pub const LIGHT: Self = Self(1);
    /// Dark colored system theme
    pub const DARK: Self = Self(2);
}
/// Unknown system theme
pub const SDL_SYSTEM_THEME_UNKNOWN: SDL_SystemTheme = SDL_SystemTheme::UNKNOWN;
/// Light colored system theme
pub const SDL_SYSTEM_THEME_LIGHT: SDL_SystemTheme = SDL_SystemTheme::LIGHT;
/// Dark colored system theme
pub const SDL_SYSTEM_THEME_DARK: SDL_SystemTheme = SDL_SystemTheme::DARK;

/// The structure that defines a display mode.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_GetFullscreenDisplayModes
/// \sa SDL_GetDesktopDisplayMode
/// \sa SDL_GetCurrentDisplayMode
/// \sa SDL_SetWindowFullscreenMode
/// \sa SDL_GetWindowFullscreenMode
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SDL_DisplayMode {
    /// the display this mode is associated with
    pub displayID: SDL_DisplayID,
    /// pixel format
    pub format: SDL_PixelFormat,
    /// width
    pub w: ::core::ffi::c_int,
    /// height
    pub h: ::core::ffi::c_int,
    /// scale converting size to pixels (e.g. a 1920x1080 mode with 2.0 scale would have 3840x2160 pixels)
    pub pixel_density: ::core::ffi::c_float,
    /// refresh rate (or 0.0f for unspecified)
    pub refresh_rate: ::core::ffi::c_float,
    /// precise refresh rate numerator (or 0 for unspecified)
    pub refresh_rate_numerator: ::core::ffi::c_int,
    /// precise refresh rate denominator
    pub refresh_rate_denominator: ::core::ffi::c_int,
    /// Private
    pub internal: *mut SDL_DisplayModeData,
}

/// Display orientation values; the way a display is rotated.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_ORIENTATION_UNKNOWN`], [`SDL_ORIENTATION_LANDSCAPE`], [`SDL_ORIENTATION_LANDSCAPE_FLIPPED`], [`SDL_ORIENTATION_PORTRAIT`], [`SDL_ORIENTATION_PORTRAIT_FLIPPED`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_DisplayOrientation(pub ::core::ffi::c_int);
impl SDL_DisplayOrientation {
    /// The display orientation can't be determined
    pub const UNKNOWN: Self = Self(0);
    /// The display is in landscape mode, with the right side up, relative to portrait mode
    pub const LANDSCAPE: Self = Self(1);
    /// The display is in landscape mode, with the left side up, relative to portrait mode
    pub const LANDSCAPE_FLIPPED: Self = Self(2);
    /// The display is in portrait mode
    pub const PORTRAIT: Self = Self(3);
    /// The display is in portrait mode, upside down
    pub const PORTRAIT_FLIPPED: Self = Self(4);
}
/// The display orientation can't be determined
pub const SDL_ORIENTATION_UNKNOWN: SDL_DisplayOrientation = SDL_DisplayOrientation::UNKNOWN;
/// The display is in landscape mode, with the right side up, relative to portrait mode
pub const SDL_ORIENTATION_LANDSCAPE: SDL_DisplayOrientation = SDL_DisplayOrientation::LANDSCAPE;
/// The display is in landscape mode, with the left side up, relative to portrait mode
pub const SDL_ORIENTATION_LANDSCAPE_FLIPPED: SDL_DisplayOrientation = SDL_DisplayOrientation::LANDSCAPE_FLIPPED;
/// The display is in portrait mode
pub const SDL_ORIENTATION_PORTRAIT: SDL_DisplayOrientation = SDL_DisplayOrientation::PORTRAIT;
/// The display is in portrait mode, upside down
pub const SDL_ORIENTATION_PORTRAIT_FLIPPED: SDL_DisplayOrientation = SDL_DisplayOrientation::PORTRAIT_FLIPPED;

/// The flags on a window.
///
/// These cover a lot of true/false, or on/off, window state. Some of it is
/// immutable after being set through SDL_CreateWindow(), some of it can be
/// changed on existing windows by the app, and some of it might be altered by
/// the user or system outside of the app's control.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_GetWindowFlags
pub type SDL_WindowFlags = Uint64;

pub const SDL_WINDOW_FULLSCREEN: ::core::primitive::u64 = 1;

pub const SDL_WINDOW_OPENGL: ::core::primitive::u64 = 2;

pub const SDL_WINDOW_OCCLUDED: ::core::primitive::u64 = 4;

pub const SDL_WINDOW_HIDDEN: ::core::primitive::u64 = 8;

pub const SDL_WINDOW_BORDERLESS: ::core::primitive::u64 = 16;

pub const SDL_WINDOW_RESIZABLE: ::core::primitive::u64 = 32;

pub const SDL_WINDOW_MINIMIZED: ::core::primitive::u64 = 64;

pub const SDL_WINDOW_MAXIMIZED: ::core::primitive::u64 = 128;

pub const SDL_WINDOW_MOUSE_GRABBED: ::core::primitive::u64 = 256;

pub const SDL_WINDOW_INPUT_FOCUS: ::core::primitive::u64 = 512;

pub const SDL_WINDOW_MOUSE_FOCUS: ::core::primitive::u64 = 1024;

pub const SDL_WINDOW_EXTERNAL: ::core::primitive::u64 = 2048;

pub const SDL_WINDOW_MODAL: ::core::primitive::u64 = 4096;

pub const SDL_WINDOW_HIGH_PIXEL_DENSITY: ::core::primitive::u64 = 8192;

pub const SDL_WINDOW_MOUSE_CAPTURE: ::core::primitive::u64 = 16384;

pub const SDL_WINDOW_MOUSE_RELATIVE_MODE: ::core::primitive::u64 = 32768;

pub const SDL_WINDOW_ALWAYS_ON_TOP: ::core::primitive::u64 = 65536;

pub const SDL_WINDOW_UTILITY: ::core::primitive::u64 = 131072;

pub const SDL_WINDOW_TOOLTIP: ::core::primitive::u64 = 262144;

pub const SDL_WINDOW_POPUP_MENU: ::core::primitive::u64 = 524288;

pub const SDL_WINDOW_KEYBOARD_GRABBED: ::core::primitive::u64 = 1048576;

pub const SDL_WINDOW_VULKAN: ::core::primitive::u64 = 268435456;

pub const SDL_WINDOW_METAL: ::core::primitive::u64 = 536870912;

pub const SDL_WINDOW_TRANSPARENT: ::core::primitive::u64 = 1073741824;

pub const SDL_WINDOW_NOT_FOCUSABLE: ::core::primitive::u64 = 2147483648;

pub const SDL_WINDOWPOS_UNDEFINED_MASK: ::core::primitive::u32 = 536805376;

pub const SDL_WINDOWPOS_CENTERED_MASK: ::core::primitive::u32 = 805240832;

/// Window flash operation.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_FLASH_CANCEL`], [`SDL_FLASH_BRIEFLY`], [`SDL_FLASH_UNTIL_FOCUSED`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_FlashOperation(pub ::core::ffi::c_int);
impl SDL_FlashOperation {
    /// Cancel any window flash state
    pub const CANCEL: Self = Self(0);
    /// Flash the window briefly to get attention
    pub const BRIEFLY: Self = Self(1);
    /// Flash the window until it gets focus
    pub const UNTIL_FOCUSED: Self = Self(2);
}
/// Cancel any window flash state
pub const SDL_FLASH_CANCEL: SDL_FlashOperation = SDL_FlashOperation::CANCEL;
/// Flash the window briefly to get attention
pub const SDL_FLASH_BRIEFLY: SDL_FlashOperation = SDL_FlashOperation::BRIEFLY;
/// Flash the window until it gets focus
pub const SDL_FLASH_UNTIL_FOCUSED: SDL_FlashOperation = SDL_FlashOperation::UNTIL_FOCUSED;

/// An opaque handle to an OpenGL context.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_GL_CreateContext
pub type SDL_GLContext = *mut SDL_GLContextState;

/// Opaque EGL types.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_EGLDisplay = *mut ::core::ffi::c_void;

pub type SDL_EGLConfig = *mut ::core::ffi::c_void;

pub type SDL_EGLSurface = *mut ::core::ffi::c_void;

pub type SDL_EGLAttrib = ::core::primitive::isize;

pub type SDL_EGLint = ::core::ffi::c_int;

/// EGL attribute initialization callback types.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_EGLAttribArrayCallback = ::core::option::Option<extern_sdlcall!(fn() -> *mut SDL_EGLAttrib)>;

pub type SDL_EGLIntArrayCallback = ::core::option::Option<extern_sdlcall!(fn() -> *mut SDL_EGLint)>;

/// An enumeration of OpenGL configuration attributes.
///
/// While you can set most OpenGL attributes normally, the attributes listed
/// above must be known before SDL creates the window that will be used with
/// the OpenGL context. These attributes are set and read with
/// SDL_GL_SetAttribute() and SDL_GL_GetAttribute().
///
/// In some cases, these attributes are minimum requests; the GL does not
/// promise to give you exactly what you asked for. It's possible to ask for a
/// 16-bit depth buffer and get a 24-bit one instead, for example, or to ask
/// for no stencil buffer and still have one available. Context creation should
/// fail if the GL can't provide your requested attributes at a minimum, but
/// you should check to see exactly what you got.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GL_RED_SIZE`], [`SDL_GL_GREEN_SIZE`], [`SDL_GL_BLUE_SIZE`], [`SDL_GL_ALPHA_SIZE`], [`SDL_GL_BUFFER_SIZE`], [`SDL_GL_DOUBLEBUFFER`], [`SDL_GL_DEPTH_SIZE`], [`SDL_GL_STENCIL_SIZE`], [`SDL_GL_ACCUM_RED_SIZE`], [`SDL_GL_ACCUM_GREEN_SIZE`], [`SDL_GL_ACCUM_BLUE_SIZE`], [`SDL_GL_ACCUM_ALPHA_SIZE`], [`SDL_GL_STEREO`], [`SDL_GL_MULTISAMPLEBUFFERS`], [`SDL_GL_MULTISAMPLESAMPLES`], [`SDL_GL_ACCELERATED_VISUAL`], [`SDL_GL_RETAINED_BACKING`], [`SDL_GL_CONTEXT_MAJOR_VERSION`], [`SDL_GL_CONTEXT_MINOR_VERSION`], [`SDL_GL_CONTEXT_FLAGS`], [`SDL_GL_CONTEXT_PROFILE_MASK`], [`SDL_GL_SHARE_WITH_CURRENT_CONTEXT`], [`SDL_GL_FRAMEBUFFER_SRGB_CAPABLE`], [`SDL_GL_CONTEXT_RELEASE_BEHAVIOR`], [`SDL_GL_CONTEXT_RESET_NOTIFICATION`], [`SDL_GL_CONTEXT_NO_ERROR`], [`SDL_GL_FLOATBUFFERS`], [`SDL_GL_EGL_PLATFORM`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GLattr(pub ::core::ffi::c_int);
impl SDL_GLattr {
    /// the minimum number of bits for the red channel of the color buffer; defaults to 3.
    pub const RED_SIZE: Self = Self(0);
    /// the minimum number of bits for the green channel of the color buffer; defaults to 3.
    pub const GREEN_SIZE: Self = Self(1);
    /// the minimum number of bits for the blue channel of the color buffer; defaults to 2.
    pub const BLUE_SIZE: Self = Self(2);
    /// the minimum number of bits for the alpha channel of the color buffer; defaults to 0.
    pub const ALPHA_SIZE: Self = Self(3);
    /// the minimum number of bits for frame buffer size; defaults to 0.
    pub const BUFFER_SIZE: Self = Self(4);
    /// whether the output is single or double buffered; defaults to double buffering on.
    pub const DOUBLEBUFFER: Self = Self(5);
    /// the minimum number of bits in the depth buffer; defaults to 16.
    pub const DEPTH_SIZE: Self = Self(6);
    /// the minimum number of bits in the stencil buffer; defaults to 0.
    pub const STENCIL_SIZE: Self = Self(7);
    /// the minimum number of bits for the red channel of the accumulation buffer; defaults to 0.
    pub const ACCUM_RED_SIZE: Self = Self(8);
    /// the minimum number of bits for the green channel of the accumulation buffer; defaults to 0.
    pub const ACCUM_GREEN_SIZE: Self = Self(9);
    /// the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0.
    pub const ACCUM_BLUE_SIZE: Self = Self(10);
    /// the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0.
    pub const ACCUM_ALPHA_SIZE: Self = Self(11);
    /// whether the output is stereo 3D; defaults to off.
    pub const STEREO: Self = Self(12);
    /// the number of buffers used for multisample anti-aliasing; defaults to 0.
    pub const MULTISAMPLEBUFFERS: Self = Self(13);
    /// the number of samples used around the current pixel used for multisample anti-aliasing.
    pub const MULTISAMPLESAMPLES: Self = Self(14);
    /// set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either.
    pub const ACCELERATED_VISUAL: Self = Self(15);
    /// not used (deprecated).
    pub const RETAINED_BACKING: Self = Self(16);
    /// OpenGL context major version.
    pub const CONTEXT_MAJOR_VERSION: Self = Self(17);
    /// OpenGL context minor version.
    pub const CONTEXT_MINOR_VERSION: Self = Self(18);
    /// some combination of 0 or more of elements of the SDL_GLcontextFlag enumeration; defaults to 0.
    pub const CONTEXT_FLAGS: Self = Self(19);
    /// type of GL context (Core, Compatibility, ES). See SDL_GLprofile; default value depends on platform.
    pub const CONTEXT_PROFILE_MASK: Self = Self(20);
    /// OpenGL context sharing; defaults to 0.
    pub const SHARE_WITH_CURRENT_CONTEXT: Self = Self(21);
    /// requests sRGB capable visual; defaults to 0.
    pub const FRAMEBUFFER_SRGB_CAPABLE: Self = Self(22);
    /// sets context the release behavior. See SDL_GLcontextReleaseFlag; defaults to FLUSH.
    pub const CONTEXT_RELEASE_BEHAVIOR: Self = Self(23);
    /// set context reset notification. See SDL_GLContextResetNotification; defaults to NO_NOTIFICATION.
    pub const CONTEXT_RESET_NOTIFICATION: Self = Self(24);
    pub const CONTEXT_NO_ERROR: Self = Self(25);
    pub const FLOATBUFFERS: Self = Self(26);
    pub const EGL_PLATFORM: Self = Self(27);
}
/// the minimum number of bits for the red channel of the color buffer; defaults to 3.
pub const SDL_GL_RED_SIZE: SDL_GLattr = SDL_GLattr::RED_SIZE;
/// the minimum number of bits for the green channel of the color buffer; defaults to 3.
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = SDL_GLattr::GREEN_SIZE;
/// the minimum number of bits for the blue channel of the color buffer; defaults to 2.
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = SDL_GLattr::BLUE_SIZE;
/// the minimum number of bits for the alpha channel of the color buffer; defaults to 0.
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = SDL_GLattr::ALPHA_SIZE;
/// the minimum number of bits for frame buffer size; defaults to 0.
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = SDL_GLattr::BUFFER_SIZE;
/// whether the output is single or double buffered; defaults to double buffering on.
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = SDL_GLattr::DOUBLEBUFFER;
/// the minimum number of bits in the depth buffer; defaults to 16.
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = SDL_GLattr::DEPTH_SIZE;
/// the minimum number of bits in the stencil buffer; defaults to 0.
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = SDL_GLattr::STENCIL_SIZE;
/// the minimum number of bits for the red channel of the accumulation buffer; defaults to 0.
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = SDL_GLattr::ACCUM_RED_SIZE;
/// the minimum number of bits for the green channel of the accumulation buffer; defaults to 0.
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = SDL_GLattr::ACCUM_GREEN_SIZE;
/// the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0.
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = SDL_GLattr::ACCUM_BLUE_SIZE;
/// the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0.
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = SDL_GLattr::ACCUM_ALPHA_SIZE;
/// whether the output is stereo 3D; defaults to off.
pub const SDL_GL_STEREO: SDL_GLattr = SDL_GLattr::STEREO;
/// the number of buffers used for multisample anti-aliasing; defaults to 0.
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = SDL_GLattr::MULTISAMPLEBUFFERS;
/// the number of samples used around the current pixel used for multisample anti-aliasing.
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = SDL_GLattr::MULTISAMPLESAMPLES;
/// set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either.
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = SDL_GLattr::ACCELERATED_VISUAL;
/// not used (deprecated).
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = SDL_GLattr::RETAINED_BACKING;
/// OpenGL context major version.
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = SDL_GLattr::CONTEXT_MAJOR_VERSION;
/// OpenGL context minor version.
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = SDL_GLattr::CONTEXT_MINOR_VERSION;
/// some combination of 0 or more of elements of the SDL_GLcontextFlag enumeration; defaults to 0.
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = SDL_GLattr::CONTEXT_FLAGS;
/// type of GL context (Core, Compatibility, ES). See SDL_GLprofile; default value depends on platform.
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = SDL_GLattr::CONTEXT_PROFILE_MASK;
/// OpenGL context sharing; defaults to 0.
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = SDL_GLattr::SHARE_WITH_CURRENT_CONTEXT;
/// requests sRGB capable visual; defaults to 0.
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = SDL_GLattr::FRAMEBUFFER_SRGB_CAPABLE;
/// sets context the release behavior. See SDL_GLcontextReleaseFlag; defaults to FLUSH.
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = SDL_GLattr::CONTEXT_RELEASE_BEHAVIOR;
/// set context reset notification. See SDL_GLContextResetNotification; defaults to NO_NOTIFICATION.
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = SDL_GLattr::CONTEXT_RESET_NOTIFICATION;
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = SDL_GLattr::CONTEXT_NO_ERROR;
pub const SDL_GL_FLOATBUFFERS: SDL_GLattr = SDL_GLattr::FLOATBUFFERS;
pub const SDL_GL_EGL_PLATFORM: SDL_GLattr = SDL_GLattr::EGL_PLATFORM;

/// Possible values to be set for the SDL_GL_CONTEXT_PROFILE_MASK attribute.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GL_CONTEXT_PROFILE_CORE`], [`SDL_GL_CONTEXT_PROFILE_COMPATIBILITY`], [`SDL_GL_CONTEXT_PROFILE_ES`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GLprofile(pub ::core::ffi::c_int);
impl SDL_GLprofile {
    pub const CORE: Self = Self(0x1000);
    pub const COMPATIBILITY: Self = Self(0x2000);
    /// GLX_CONTEXT_ES2_PROFILE_BIT_EXT
    pub const ES: Self = Self(0x4000);
}
pub const SDL_GL_CONTEXT_PROFILE_CORE: SDL_GLprofile = SDL_GLprofile::CORE;
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: SDL_GLprofile = SDL_GLprofile::COMPATIBILITY;
/// GLX_CONTEXT_ES2_PROFILE_BIT_EXT
pub const SDL_GL_CONTEXT_PROFILE_ES: SDL_GLprofile = SDL_GLprofile::ES;

/// Possible values to be set for the SDL_GL_CONTEXT_FLAGS attribute.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GL_CONTEXT_DEBUG_FLAG`], [`SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG`], [`SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG`], [`SDL_GL_CONTEXT_RESET_ISOLATION_FLAG`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GLcontextFlag(pub ::core::ffi::c_int);
impl SDL_GLcontextFlag {
    pub const DEBUG_FLAG: Self = Self(0x1000);
    pub const FORWARD_COMPATIBLE_FLAG: Self = Self(0x2000);
    pub const ROBUST_ACCESS_FLAG: Self = Self(0x4000);
    pub const RESET_ISOLATION_FLAG: Self = Self(0x8000);
}
pub const SDL_GL_CONTEXT_DEBUG_FLAG: SDL_GLcontextFlag = SDL_GLcontextFlag::DEBUG_FLAG;
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: SDL_GLcontextFlag = SDL_GLcontextFlag::FORWARD_COMPATIBLE_FLAG;
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: SDL_GLcontextFlag = SDL_GLcontextFlag::ROBUST_ACCESS_FLAG;
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: SDL_GLcontextFlag = SDL_GLcontextFlag::RESET_ISOLATION_FLAG;

/// Possible values to be set for the SDL_GL_CONTEXT_RELEASE_BEHAVIOR
/// attribute.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE`], [`SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GLcontextReleaseFlag(pub ::core::ffi::c_int);
impl SDL_GLcontextReleaseFlag {
    pub const NONE: Self = Self(0x0000);
    pub const FLUSH: Self = Self(0x1000);
}
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE: SDL_GLcontextReleaseFlag = SDL_GLcontextReleaseFlag::NONE;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: SDL_GLcontextReleaseFlag = SDL_GLcontextReleaseFlag::FLUSH;

/// Possible values to be set SDL_GL_CONTEXT_RESET_NOTIFICATION attribute.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_GL_CONTEXT_RESET_NO_NOTIFICATION`], [`SDL_GL_CONTEXT_RESET_LOSE_CONTEXT`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_GLContextResetNotification(pub ::core::ffi::c_int);
impl SDL_GLContextResetNotification {
    pub const NO_NOTIFICATION: Self = Self(0x0000);
    pub const LOSE_CONTEXT: Self = Self(0x1000);
}
pub const SDL_GL_CONTEXT_RESET_NO_NOTIFICATION: SDL_GLContextResetNotification = SDL_GLContextResetNotification::NO_NOTIFICATION;
pub const SDL_GL_CONTEXT_RESET_LOSE_CONTEXT: SDL_GLContextResetNotification = SDL_GLContextResetNotification::LOSE_CONTEXT;

extern_sdlcall! {{
    /// Get the number of video drivers compiled into SDL.
    ///
    /// \returns the number of built in video drivers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetVideoDriver
    pub fn SDL_GetNumVideoDrivers() -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the name of a built in video driver.
    ///
    /// The video drivers are presented in the order in which they are normally
    /// checked during initialization.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "cocoa",
    /// "x11" or "windows". These never have Unicode characters, and are not meant
    /// to be proper names.
    ///
    /// \param index the index of a video driver.
    /// \returns the name of the video driver with the given **index**.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetNumVideoDrivers
    pub fn SDL_GetVideoDriver(index: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the name of the currently initialized video driver.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "cocoa",
    /// "x11" or "windows". These never have Unicode characters, and are not meant
    /// to be proper names.
    ///
    /// \returns the name of the current video driver or NULL if no driver has been
    ///          initialized.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetNumVideoDrivers
    /// \sa SDL_GetVideoDriver
    pub fn SDL_GetCurrentVideoDriver() -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the current system theme.
    ///
    /// \returns the current system theme, light, dark, or unknown.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSystemTheme() -> SDL_SystemTheme;
}}

extern_sdlcall! {{
    /// Get a list of currently connected displays.
    ///
    /// \param count a pointer filled in with the number of displays returned, may
    ///              be NULL.
    /// \returns a 0 terminated array of display instance IDs or NULL on failure;
    ///          call SDL_GetError() for more information. This should be freed
    ///          with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetDisplays(count: *mut ::core::ffi::c_int) -> *mut SDL_DisplayID;
}}

extern_sdlcall! {{
    /// Return the primary display.
    ///
    /// \returns the instance ID of the primary display on success or 0 on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetPrimaryDisplay() -> SDL_DisplayID;
}}

extern_sdlcall! {{
    /// Get the properties associated with a display.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN`: true if the display has HDR
    ///   headroom above the SDR white point. This is for informational and
    ///   diagnostic purposes only, as not all platforms provide this information
    ///   at the display level.
    ///
    /// On KMS/DRM:
    ///
    /// - `SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER`: the "panel
    ///   orientation" property for the display in degrees of clockwise rotation.
    ///   Note that this is provided only as a hint, and the application is
    ///   responsible for any coordinate transformations needed to conform to the
    ///   requested display orientation.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetDisplayProperties(displayID: SDL_DisplayID) -> SDL_PropertiesID;
}}

pub const SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.display.HDR_enabled\0") };

pub const SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.display.KMSDRM.panel_orientation\0") };

extern_sdlcall! {{
    /// Get the name of a display in UTF-8 encoding.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns the name of a display or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayName(displayID: SDL_DisplayID) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the desktop area represented by a display.
    ///
    /// The primary display is always located at (0,0).
    ///
    /// \param displayID the instance ID of the display to query.
    /// \param rect the SDL_Rect structure filled in with the display bounds.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplayUsableBounds
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayBounds(displayID: SDL_DisplayID, rect: *mut SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the usable desktop area represented by a display, in screen
    /// coordinates.
    ///
    /// This is the same area as SDL_GetDisplayBounds() reports, but with portions
    /// reserved by the system removed. For example, on Apple's macOS, this
    /// subtracts the area occupied by the menu bar and dock.
    ///
    /// Setting a window to be fullscreen generally bypasses these unusable areas,
    /// so these are good guidelines for the maximum space available to a
    /// non-fullscreen window.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \param rect the SDL_Rect structure filled in with the display bounds.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplayBounds
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayUsableBounds(displayID: SDL_DisplayID, rect: *mut SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the orientation of a display when it is unrotated.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns the SDL_DisplayOrientation enum value of the display, or
    ///          `SDL_ORIENTATION_UNKNOWN` if it isn't available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetNaturalDisplayOrientation(displayID: SDL_DisplayID) -> SDL_DisplayOrientation;
}}

extern_sdlcall! {{
    /// Get the orientation of a display.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns the SDL_DisplayOrientation enum value of the display, or
    ///          `SDL_ORIENTATION_UNKNOWN` if it isn't available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetCurrentDisplayOrientation(displayID: SDL_DisplayID) -> SDL_DisplayOrientation;
}}

extern_sdlcall! {{
    /// Get the content scale of a display.
    ///
    /// The content scale is the expected scale for content based on the DPI
    /// settings of the display. For example, a 4K display might have a 2.0 (200%)
    /// display scale, which means that the user expects UI elements to be twice as
    /// big on this display, to aid in readability.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns the content scale of the display, or 0.0f on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayContentScale(displayID: SDL_DisplayID) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Get a list of fullscreen display modes available on a display.
    ///
    /// The display modes are sorted in this priority:
    ///
    /// - w -> largest to smallest
    /// - h -> largest to smallest
    /// - bits per pixel -> more colors to fewer colors
    /// - packed pixel layout -> largest to smallest
    /// - refresh rate -> highest to lowest
    /// - pixel density -> lowest to highest
    ///
    /// \param displayID the instance ID of the display to query.
    /// \param count a pointer filled in with the number of display modes returned,
    ///              may be NULL.
    /// \returns a NULL terminated array of display mode pointers or NULL on
    ///          failure; call SDL_GetError() for more information. This is a
    ///          single allocation that should be freed with SDL_free() when it is
    ///          no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    pub fn SDL_GetFullscreenDisplayModes(displayID: SDL_DisplayID, count: *mut ::core::ffi::c_int) -> *mut *mut SDL_DisplayMode;
}}

extern_sdlcall! {{
    /// Get the closest match to the requested display mode.
    ///
    /// The available display modes are scanned and `closest` is filled in with the
    /// closest mode matching the requested mode and returned. The mode format and
    /// refresh rate default to the desktop mode if they are set to 0. The modes
    /// are scanned with size being first priority, format being second priority,
    /// and finally checking the refresh rate. If all the available modes are too
    /// small, then NULL is returned.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \param w the width in pixels of the desired display mode.
    /// \param h the height in pixels of the desired display mode.
    /// \param refresh_rate the refresh rate of the desired display mode, or 0.0f
    ///                     for the desktop refresh rate.
    /// \param include_high_density_modes boolean to include high density modes in
    ///                                   the search.
    /// \param mode a pointer filled in with the closest display mode equal to or
    ///             larger than the desired mode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplays
    /// \sa SDL_GetFullscreenDisplayModes
    pub fn SDL_GetClosestFullscreenDisplayMode(displayID: SDL_DisplayID, w: ::core::ffi::c_int, h: ::core::ffi::c_int, refresh_rate: ::core::ffi::c_float, include_high_density_modes: SDL_bool, mode: *mut SDL_DisplayMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get information about the desktop's display mode.
    ///
    /// There's a difference between this function and SDL_GetCurrentDisplayMode()
    /// when SDL runs fullscreen and has changed the resolution. In that case this
    /// function will return the previous native display mode, and not the current
    /// display mode.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns a pointer to the desktop display mode or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetCurrentDisplayMode
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDesktopDisplayMode(displayID: SDL_DisplayID) -> *const SDL_DisplayMode;
}}

extern_sdlcall! {{
    /// Get information about the current display mode.
    ///
    /// There's a difference between this function and SDL_GetDesktopDisplayMode()
    /// when SDL runs fullscreen and has changed the resolution. In that case this
    /// function will return the current display mode, and not the previous native
    /// display mode.
    ///
    /// \param displayID the instance ID of the display to query.
    /// \returns a pointer to the desktop display mode or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDesktopDisplayMode
    /// \sa SDL_GetDisplays
    pub fn SDL_GetCurrentDisplayMode(displayID: SDL_DisplayID) -> *const SDL_DisplayMode;
}}

extern_sdlcall! {{
    /// Get the display containing a point.
    ///
    /// \param point the point to query.
    /// \returns the instance ID of the display containing the point or 0 on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplayBounds
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayForPoint(point: *const SDL_Point) -> SDL_DisplayID;
}}

extern_sdlcall! {{
    /// Get the display primarily containing a rect.
    ///
    /// \param rect the rect to query.
    /// \returns the instance ID of the display entirely containing the rect or
    ///          closest to the center of the rect on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplayBounds
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayForRect(rect: *const SDL_Rect) -> SDL_DisplayID;
}}

extern_sdlcall! {{
    /// Get the display associated with a window.
    ///
    /// \param window the window to query.
    /// \returns the instance ID of the display containing the center of the window
    ///          on success or 0 on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetDisplayBounds
    /// \sa SDL_GetDisplays
    pub fn SDL_GetDisplayForWindow(window: *mut SDL_Window) -> SDL_DisplayID;
}}

extern_sdlcall! {{
    /// Get the pixel density of a window.
    ///
    /// This is a ratio of pixel size to window size. For example, if the window is
    /// 1920x1080 and it has a high density back buffer of 3840x2160 pixels, it
    /// would have a pixel density of 2.0.
    ///
    /// \param window the window to query.
    /// \returns the pixel density or 0.0f on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowDisplayScale
    pub fn SDL_GetWindowPixelDensity(window: *mut SDL_Window) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Get the content display scale relative to a window's pixel size.
    ///
    /// This is a combination of the window pixel density and the display content
    /// scale, and is the expected scale for displaying content in this window. For
    /// example, if a 3840x2160 window had a display scale of 2.0, the user expects
    /// the content to take twice as many pixels and be the same physical size as
    /// if it were being displayed in a 1920x1080 window with a display scale of
    /// 1.0.
    ///
    /// Conceptually this value corresponds to the scale display setting, and is
    /// updated when that setting is changed, or the window moves to a display with
    /// a different scale setting.
    ///
    /// \param window the window to query.
    /// \returns the display scale, or 0.0f on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindowDisplayScale(window: *mut SDL_Window) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Set the display mode to use when a window is visible and fullscreen.
    ///
    /// This only affects the display mode used when the window is fullscreen. To
    /// change the window size when the window is not fullscreen, use
    /// SDL_SetWindowSize().
    ///
    /// If the window is currently in the fullscreen state, this request is
    /// asynchronous on some windowing systems and the new mode dimensions may not
    /// be applied immediately upon the return of this function. If an immediate
    /// change is required, call SDL_SyncWindow() to block until the changes have
    /// taken effect.
    ///
    /// When the new mode takes effect, an SDL_EVENT_WINDOW_RESIZED and/or an
    /// SDL_EVENT_WINDOOW_PIXEL_SIZE_CHANGED event will be emitted with the new
    /// mode dimensions.
    ///
    /// \param window the window to affect.
    /// \param mode a pointer to the display mode to use, which can be NULL for
    ///             borderless fullscreen desktop mode, or one of the fullscreen
    ///             modes returned by SDL_GetFullscreenDisplayModes() to set an
    ///             exclusive fullscreen mode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFullscreenMode
    /// \sa SDL_SetWindowFullscreen
    /// \sa SDL_SyncWindow
    pub fn SDL_SetWindowFullscreenMode(window: *mut SDL_Window, mode: *const SDL_DisplayMode) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Query the display mode to use when a window is visible at fullscreen.
    ///
    /// \param window the window to query.
    /// \returns a pointer to the exclusive fullscreen mode to use or NULL for
    ///          borderless fullscreen desktop mode.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowFullscreenMode
    /// \sa SDL_SetWindowFullscreen
    pub fn SDL_GetWindowFullscreenMode(window: *mut SDL_Window) -> *const SDL_DisplayMode;
}}

extern_sdlcall! {{
    /// Get the raw ICC profile data for the screen the window is currently on.
    ///
    /// \param window the window to query.
    /// \param size the size of the ICC profile.
    /// \returns the raw ICC profile data on success or NULL on failure; call
    ///          SDL_GetError() for more information. This should be freed with
    ///          SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindowICCProfile(window: *mut SDL_Window, size: *mut ::core::primitive::usize) -> *mut ::core::ffi::c_void;
}}

extern_sdlcall! {{
    /// Get the pixel format associated with the window.
    ///
    /// \param window the window to query.
    /// \returns the pixel format of the window on success or
    ///          SDL_PIXELFORMAT_UNKNOWN on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindowPixelFormat(window: *mut SDL_Window) -> SDL_PixelFormat;
}}

extern_sdlcall! {{
    /// Get a list of valid windows.
    ///
    /// \param count a pointer filled in with the number of windows returned, may
    ///              be NULL.
    /// \returns a NULL terminated array of SDL_Window pointers or NULL on failure;
    ///          call SDL_GetError() for more information. This is a single
    ///          allocation that should be freed with SDL_free() when it is no
    ///          longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindows(count: *mut ::core::ffi::c_int) -> *mut *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Create a window with the specified dimensions and flags.
    ///
    /// `flags` may be any of the following OR'd together:
    ///
    /// - `SDL_WINDOW_FULLSCREEN`: fullscreen window at desktop resolution
    /// - `SDL_WINDOW_OPENGL`: window usable with an OpenGL context
    /// - `SDL_WINDOW_OCCLUDED`: window partially or completely obscured by another
    ///   window
    /// - `SDL_WINDOW_HIDDEN`: window is not visible
    /// - `SDL_WINDOW_BORDERLESS`: no window decoration
    /// - `SDL_WINDOW_RESIZABLE`: window can be resized
    /// - `SDL_WINDOW_MINIMIZED`: window is minimized
    /// - `SDL_WINDOW_MAXIMIZED`: window is maximized
    /// - `SDL_WINDOW_MOUSE_GRABBED`: window has grabbed mouse focus
    /// - `SDL_WINDOW_INPUT_FOCUS`: window has input focus
    /// - `SDL_WINDOW_MOUSE_FOCUS`: window has mouse focus
    /// - `SDL_WINDOW_EXTERNAL`: window not created by SDL
    /// - `SDL_WINDOW_MODAL`: window is modal
    /// - `SDL_WINDOW_HIGH_PIXEL_DENSITY`: window uses high pixel density back
    ///   buffer if possible
    /// - `SDL_WINDOW_MOUSE_CAPTURE`: window has mouse captured (unrelated to
    ///   MOUSE_GRABBED)
    /// - `SDL_WINDOW_ALWAYS_ON_TOP`: window should always be above others
    /// - `SDL_WINDOW_UTILITY`: window should be treated as a utility window, not
    ///   showing in the task bar and window list
    /// - `SDL_WINDOW_TOOLTIP`: window should be treated as a tooltip and does not
    ///   get mouse or keyboard focus, requires a parent window
    /// - `SDL_WINDOW_POPUP_MENU`: window should be treated as a popup menu,
    ///   requires a parent window
    /// - `SDL_WINDOW_KEYBOARD_GRABBED`: window has grabbed keyboard input
    /// - `SDL_WINDOW_VULKAN`: window usable with a Vulkan instance
    /// - `SDL_WINDOW_METAL`: window usable with a Metal instance
    /// - `SDL_WINDOW_TRANSPARENT`: window with transparent buffer
    /// - `SDL_WINDOW_NOT_FOCUSABLE`: window should not be focusable
    ///
    /// The SDL_Window is implicitly shown if SDL_WINDOW_HIDDEN is not set.
    ///
    /// On Apple's macOS, you **must** set the NSHighResolutionCapable Info.plist
    /// property to YES, otherwise you will not receive a High-DPI OpenGL canvas.
    ///
    /// The window pixel size may differ from its window coordinate size if the
    /// window is on a high pixel density display. Use SDL_GetWindowSize() to query
    /// the client area's size in window coordinates, and
    /// SDL_GetWindowSizeInPixels() or SDL_GetRenderOutputSize() to query the
    /// drawable size in pixels. Note that the drawable size can vary after the
    /// window is created and should be queried again if you get an
    /// SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event.
    ///
    /// If the window is created with any of the SDL_WINDOW_OPENGL or
    /// SDL_WINDOW_VULKAN flags, then the corresponding LoadLibrary function
    /// (SDL_GL_LoadLibrary or SDL_Vulkan_LoadLibrary) is called and the
    /// corresponding UnloadLibrary function is called by SDL_DestroyWindow().
    ///
    /// If SDL_WINDOW_VULKAN is specified and there isn't a working Vulkan driver,
    /// SDL_CreateWindow() will fail because SDL_Vulkan_LoadLibrary() will fail.
    ///
    /// If SDL_WINDOW_METAL is specified on an OS that does not support Metal,
    /// SDL_CreateWindow() will fail.
    ///
    /// If you intend to use this window with an SDL_Renderer, you should use
    /// SDL_CreateWindowAndRenderer() instead of this function, to avoid window
    /// flicker.
    ///
    /// On non-Apple devices, SDL requires you to either not link to the Vulkan
    /// loader or link to a dynamic library version. This limitation may be removed
    /// in a future version of SDL.
    ///
    /// \param title the title of the window, in UTF-8 encoding.
    /// \param w the width of the window.
    /// \param h the height of the window.
    /// \param flags 0, or one or more SDL_WindowFlags OR'd together.
    /// \returns the window that was created or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreatePopupWindow
    /// \sa SDL_CreateWindowWithProperties
    /// \sa SDL_DestroyWindow
    pub fn SDL_CreateWindow(title: *const ::core::ffi::c_char, w: ::core::ffi::c_int, h: ::core::ffi::c_int, flags: SDL_WindowFlags) -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Create a child popup window of the specified parent window.
    ///
    /// 'flags' **must** contain exactly one of the following: -
    /// 'SDL_WINDOW_TOOLTIP': The popup window is a tooltip and will not pass any
    /// input events. - 'SDL_WINDOW_POPUP_MENU': The popup window is a popup menu.
    /// The topmost popup menu will implicitly gain the keyboard focus.
    ///
    /// The following flags are not relevant to popup window creation and will be
    /// ignored:
    ///
    /// - 'SDL_WINDOW_MINIMIZED'
    /// - 'SDL_WINDOW_MAXIMIZED'
    /// - 'SDL_WINDOW_FULLSCREEN'
    /// - 'SDL_WINDOW_BORDERLESS'
    ///
    /// The parent parameter **must** be non-null and a valid window. The parent of
    /// a popup window can be either a regular, toplevel window, or another popup
    /// window.
    ///
    /// Popup windows cannot be minimized, maximized, made fullscreen, raised,
    /// flash, be made a modal window, be the parent of a modal window, or grab the
    /// mouse and/or keyboard. Attempts to do so will fail.
    ///
    /// Popup windows implicitly do not have a border/decorations and do not appear
    /// on the taskbar/dock or in lists of windows such as alt-tab menus.
    ///
    /// If a parent window is hidden, any child popup windows will be recursively
    /// hidden as well. Child popup windows not explicitly hidden will be restored
    /// when the parent is shown.
    ///
    /// If the parent window is destroyed, any child popup windows will be
    /// recursively destroyed as well.
    ///
    /// \param parent the parent of the window, must not be NULL.
    /// \param offset_x the x position of the popup window relative to the origin
    ///                 of the parent.
    /// \param offset_y the y position of the popup window relative to the origin
    ///                 of the parent window.
    /// \param w the width of the window.
    /// \param h the height of the window.
    /// \param flags SDL_WINDOW_TOOLTIP or SDL_WINDOW_POPUP_MENU, and zero or more
    ///              additional SDL_WindowFlags OR'd together.
    /// \returns the window that was created or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateWindow
    /// \sa SDL_CreateWindowWithProperties
    /// \sa SDL_DestroyWindow
    /// \sa SDL_GetWindowParent
    pub fn SDL_CreatePopupWindow(parent: *mut SDL_Window, offset_x: ::core::ffi::c_int, offset_y: ::core::ffi::c_int, w: ::core::ffi::c_int, h: ::core::ffi::c_int, flags: SDL_WindowFlags) -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Create a window with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN`: true if the window should
    ///   be always on top
    /// - `SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN`: true if the window has no
    ///   window decoration
    /// - `SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN`: true if the
    ///   window will be used with an externally managed graphics context.
    /// - `SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN`: true if the window should
    ///   accept keyboard input (defaults true)
    /// - `SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN`: true if the window should
    ///   start in fullscreen mode at desktop resolution
    /// - `SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER`: the height of the window
    /// - `SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN`: true if the window should start
    ///   hidden
    /// - `SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN`: true if the window
    ///   uses a high pixel density buffer if possible
    /// - `SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN`: true if the window should
    ///   start maximized
    /// - `SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN`: true if the window is a popup menu
    /// - `SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN`: true if the window will be used
    ///   with Metal rendering
    /// - `SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN`: true if the window should
    ///   start minimized
    /// - `SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN`: true if the window is modal to
    ///   its parent
    /// - `SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN`: true if the window starts
    ///   with grabbed mouse focus
    /// - `SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN`: true if the window will be used
    ///   with OpenGL rendering
    /// - `SDL_PROP_WINDOW_CREATE_PARENT_POINTER`: an SDL_Window that will be the
    ///   parent of this window, required for windows with the "toolip", "menu",
    ///   and "modal" properties
    /// - `SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN`: true if the window should be
    ///   resizable
    /// - `SDL_PROP_WINDOW_CREATE_TITLE_STRING`: the title of the window, in UTF-8
    ///   encoding
    /// - `SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN`: true if the window show
    ///   transparent in the areas with alpha of 0
    /// - `SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN`: true if the window is a tooltip
    /// - `SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN`: true if the window is a utility
    ///   window, not showing in the task bar and window list
    /// - `SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN`: true if the window will be used
    ///   with Vulkan rendering
    /// - `SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER`: the width of the window
    /// - `SDL_PROP_WINDOW_CREATE_X_NUMBER`: the x position of the window, or
    ///   `SDL_WINDOWPOS_CENTERED`, defaults to `SDL_WINDOWPOS_UNDEFINED`. This is
    ///   relative to the parent for windows with the "parent" property set.
    /// - `SDL_PROP_WINDOW_CREATE_Y_NUMBER`: the y position of the window, or
    ///   `SDL_WINDOWPOS_CENTERED`, defaults to `SDL_WINDOWPOS_UNDEFINED`. This is
    ///   relative to the parent for windows with the "parent" property set.
    ///
    /// These are additional supported properties on macOS:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER`: the
    ///   `(__unsafe_unretained)` NSWindow associated with the window, if you want
    ///   to wrap an existing window.
    /// - `SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER`: the `(__unsafe_unretained)`
    ///   NSView associated with the window, defaults to `[window contentView]`
    ///
    /// These are additional supported properties on Wayland:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN` - true if
    ///   the application wants to use the Wayland surface for a custom role and
    ///   does not want it attached to an XDG toplevel window. See
    ///   [README/wayland](README/wayland) for more information on using custom
    ///   surfaces.
    /// - `SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN` - true if the
    ///   application wants an associated `wl_egl_window` object to be created,
    ///   even if the window does not have the OpenGL property or flag set.
    /// - `SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER` - the wl_surface
    ///   associated with the window, if you want to wrap an existing window. See
    ///   [README/wayland](README/wayland) for more information.
    ///
    /// These are additional supported properties on Windows:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER`: the HWND associated with the
    ///   window, if you want to wrap an existing window.
    /// - `SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER`: optional,
    ///   another window to share pixel format with, useful for OpenGL windows
    ///
    /// These are additional supported properties with X11:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER`: the X11 Window associated
    ///   with the window, if you want to wrap an existing window.
    ///
    /// The window is implicitly shown if the "hidden" property is not set.
    ///
    /// Windows with the "tooltip" and "menu" properties are popup windows and have
    /// the behaviors and guidelines outlined in SDL_CreatePopupWindow().
    ///
    /// If this window is being created to be used with an SDL_Renderer, you should
    /// not add a graphics API specific property
    /// (`SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN`, etc), as SDL will handle that
    /// internally when it chooses a renderer. However, SDL might need to recreate
    /// your window at that point, which may cause the window to appear briefly,
    /// and then flicker as it is recreated. The correct approach to this is to
    /// create the window with the `SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN` property
    /// set to true, then create the renderer, then show the window with
    /// SDL_ShowWindow().
    ///
    /// \param props the properties to use.
    /// \returns the window that was created or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProperties
    /// \sa SDL_CreateWindow
    /// \sa SDL_DestroyWindow
    pub fn SDL_CreateWindowWithProperties(props: SDL_PropertiesID) -> *mut SDL_Window;
}}

pub const SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.always_on_top\0") };

pub const SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.borderless\0") };

pub const SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.focusable\0") };

pub const SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.external_graphics_context\0") };

pub const SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.flags\0") };

pub const SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.fullscreen\0") };

pub const SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.height\0") };

pub const SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.hidden\0") };

pub const SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.high_pixel_density\0") };

pub const SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.maximized\0") };

pub const SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.menu\0") };

pub const SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.metal\0") };

pub const SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.minimized\0") };

pub const SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.modal\0") };

pub const SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.mouse_grabbed\0") };

pub const SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.opengl\0") };

pub const SDL_PROP_WINDOW_CREATE_PARENT_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.parent\0") };

pub const SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.resizable\0") };

pub const SDL_PROP_WINDOW_CREATE_TITLE_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.title\0") };

pub const SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.transparent\0") };

pub const SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.tooltip\0") };

pub const SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.utility\0") };

pub const SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.vulkan\0") };

pub const SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.width\0") };

pub const SDL_PROP_WINDOW_CREATE_X_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.x\0") };

pub const SDL_PROP_WINDOW_CREATE_Y_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.y\0") };

pub const SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.cocoa.window\0") };

pub const SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.cocoa.view\0") };

pub const SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.wayland.surface_role_custom\0") };

pub const SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.wayland.create_egl_window\0") };

pub const SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.wayland.wl_surface\0") };

pub const SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.win32.hwnd\0") };

pub const SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.win32.pixel_format_hwnd\0") };

pub const SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.create.x11.window\0") };

extern_sdlcall! {{
    /// Get the numeric ID of a window.
    ///
    /// The numeric ID is what SDL_WindowEvent references, and is necessary to map
    /// these events to specific SDL_Window objects.
    ///
    /// \param window the window to query.
    /// \returns the ID of the window on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFromID
    pub fn SDL_GetWindowID(window: *mut SDL_Window) -> SDL_WindowID;
}}

extern_sdlcall! {{
    /// Get a window from a stored ID.
    ///
    /// The numeric ID is what SDL_WindowEvent references, and is necessary to map
    /// these events to specific SDL_Window objects.
    ///
    /// \param id the ID of the window.
    /// \returns the window associated with `id` or NULL if it doesn't exist; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowID
    pub fn SDL_GetWindowFromID(id: SDL_WindowID) -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Get parent of a window.
    ///
    /// \param window the window to query.
    /// \returns the parent of the window on success or NULL if the window has no
    ///          parent.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreatePopupWindow
    pub fn SDL_GetWindowParent(window: *mut SDL_Window) -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Get the properties associated with a window.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_WINDOW_SHAPE_POINTER`: the surface associated with a shaped
    ///   window
    /// - `SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN`: true if the window has HDR
    ///   headroom above the SDR white point. This property can change dynamically
    ///   when SDL_EVENT_WINDOW_HDR_STATE_CHANGED is sent.
    /// - `SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT`: the value of SDR white in the
    ///   SDL_COLORSPACE_SRGB_LINEAR colorspace. On Windows this corresponds to the
    ///   SDR white level in scRGB colorspace, and on Apple platforms this is
    ///   always 1.0 for EDR content. This property can change dynamically when
    ///   SDL_EVENT_WINDOW_HDR_STATE_CHANGED is sent.
    /// - `SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT`: the additional high dynamic range
    ///   that can be displayed, in terms of the SDR white point. When HDR is not
    ///   enabled, this will be 1.0. This property can change dynamically when
    ///   SDL_EVENT_WINDOW_HDR_STATE_CHANGED is sent.
    ///
    /// On Android:
    ///
    /// - `SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER`: the ANativeWindow associated
    ///   with the window
    /// - `SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER`: the EGLSurface associated with
    ///   the window
    ///
    /// On iOS:
    ///
    /// - `SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER`: the `(__unsafe_unretained)`
    ///   UIWindow associated with the window
    /// - `SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER`: the NSInteger tag
    ///   assocated with metal views on the window
    /// - `SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER`: the OpenGL view's
    ///   framebuffer object. It must be bound when rendering to the screen using
    ///   OpenGL.
    /// - `SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER`: the OpenGL view's
    ///   renderbuffer object. It must be bound when SDL_GL_SwapWindow is called.
    /// - `SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER`: the OpenGL
    ///   view's resolve framebuffer, when MSAA is used.
    ///
    /// On KMS/DRM:
    ///
    /// - `SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER`: the device index associated
    ///   with the window (e.g. the X in /dev/dri/cardX)
    /// - `SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER`: the DRM FD associated with the
    ///   window
    /// - `SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER`: the GBM device associated
    ///   with the window
    ///
    /// On macOS:
    ///
    /// - `SDL_PROP_WINDOW_COCOA_WINDOW_POINTER`: the `(__unsafe_unretained)`
    ///   NSWindow associated with the window
    /// - `SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER`: the NSInteger tag
    ///   assocated with metal views on the window
    ///
    /// On Vivante:
    ///
    /// - `SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER`: the EGLNativeDisplayType
    ///   associated with the window
    /// - `SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER`: the EGLNativeWindowType
    ///   associated with the window
    /// - `SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER`: the EGLSurface associated with
    ///   the window
    ///
    /// On UWP:
    ///
    /// - `SDL_PROP_WINDOW_WINRT_WINDOW_POINTER`: the IInspectable CoreWindow
    ///   associated with the window
    ///
    /// On Windows:
    ///
    /// - `SDL_PROP_WINDOW_WIN32_HWND_POINTER`: the HWND associated with the window
    /// - `SDL_PROP_WINDOW_WIN32_HDC_POINTER`: the HDC associated with the window
    /// - `SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER`: the HINSTANCE associated with
    ///   the window
    ///
    /// On Wayland:
    ///
    /// Note: The `xdg_*` window objects do not internally persist across window
    /// show/hide calls. They will be null if the window is hidden and must be
    /// queried each time it is shown.
    ///
    /// - `SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER`: the wl_display associated with
    ///   the window
    /// - `SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER`: the wl_surface associated with
    ///   the window
    /// - `SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER`: the wl_egl_window
    ///   associated with the window
    /// - `SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER`: the xdg_surface associated
    ///   with the window
    /// - `SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER`: the xdg_toplevel role
    ///   associated with the window
    /// - 'SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING': the export
    ///   handle associated with the window
    /// - `SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER`: the xdg_popup role
    ///   associated with the window
    /// - `SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER`: the xdg_positioner
    ///   associated with the window, in popup mode
    ///
    /// On X11:
    ///
    /// - `SDL_PROP_WINDOW_X11_DISPLAY_POINTER`: the X11 Display associated with
    ///   the window
    /// - `SDL_PROP_WINDOW_X11_SCREEN_NUMBER`: the screen number associated with
    ///   the window
    /// - `SDL_PROP_WINDOW_X11_WINDOW_NUMBER`: the X11 Window associated with the
    ///   window
    ///
    /// \param window the window to query.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindowProperties(window: *mut SDL_Window) -> SDL_PropertiesID;
}}

pub const SDL_PROP_WINDOW_SHAPE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.shape\0") };

pub const SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.HDR_enabled\0") };

pub const SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.SDR_white_level\0") };

pub const SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.HDR_headroom\0") };

pub const SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.android.window\0") };

pub const SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.android.surface\0") };

pub const SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.uikit.window\0") };

pub const SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.uikit.metal_view_tag\0") };

pub const SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.uikit.opengl.framebuffer\0") };

pub const SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.uikit.opengl.renderbuffer\0") };

pub const SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.uikit.opengl.resolve_framebuffer\0") };

pub const SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.kmsdrm.dev_index\0") };

pub const SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.kmsdrm.drm_fd\0") };

pub const SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.kmsdrm.gbm_dev\0") };

pub const SDL_PROP_WINDOW_COCOA_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.cocoa.window\0") };

pub const SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.cocoa.metal_view_tag\0") };

pub const SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.vivante.display\0") };

pub const SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.vivante.window\0") };

pub const SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.vivante.surface\0") };

pub const SDL_PROP_WINDOW_WINRT_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.winrt.window\0") };

pub const SDL_PROP_WINDOW_WIN32_HWND_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.win32.hwnd\0") };

pub const SDL_PROP_WINDOW_WIN32_HDC_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.win32.hdc\0") };

pub const SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.win32.instance\0") };

pub const SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.display\0") };

pub const SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.surface\0") };

pub const SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.egl_window\0") };

pub const SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.xdg_surface\0") };

pub const SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.xdg_toplevel\0") };

pub const SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.xdg_toplevel_export_handle\0") };

pub const SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.xdg_popup\0") };

pub const SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.wayland.xdg_positioner\0") };

pub const SDL_PROP_WINDOW_X11_DISPLAY_POINTER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.x11.display\0") };

pub const SDL_PROP_WINDOW_X11_SCREEN_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.x11.screen\0") };

pub const SDL_PROP_WINDOW_X11_WINDOW_NUMBER: &::core::ffi::CStr = unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.window.x11.window\0") };

extern_sdlcall! {{
    /// Get the window flags.
    ///
    /// \param window the window to query.
    /// \returns a mask of the SDL_WindowFlags associated with `window`.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateWindow
    /// \sa SDL_HideWindow
    /// \sa SDL_MaximizeWindow
    /// \sa SDL_MinimizeWindow
    /// \sa SDL_SetWindowFullscreen
    /// \sa SDL_SetWindowMouseGrab
    /// \sa SDL_ShowWindow
    pub fn SDL_GetWindowFlags(window: *mut SDL_Window) -> SDL_WindowFlags;
}}

extern_sdlcall! {{
    /// Set the title of a window.
    ///
    /// This string is expected to be in UTF-8 encoding.
    ///
    /// \param window the window to change.
    /// \param title the desired window title in UTF-8 format.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowTitle
    pub fn SDL_SetWindowTitle(window: *mut SDL_Window, title: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the title of a window.
    ///
    /// \param window the window to query.
    /// \returns the title of the window in UTF-8 format or "" if there is no
    ///          title.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowTitle
    pub fn SDL_GetWindowTitle(window: *mut SDL_Window) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Set the icon for a window.
    ///
    /// If this function is passed a surface with alternate representations, the
    /// surface will be interpreted as the content to be used for 100% display
    /// scale, and the alternate representations will be used for high DPI
    /// situations. For example, if the original surface is 32x32, then on a 2x
    /// macOS display or 200% display scale on Windows, a 64x64 version of the
    /// image will be used, if available. If a matching version of the image isn't
    /// available, the closest larger size image will be downscaled to the
    /// appropriate size and be used instead, if available. Otherwise, the closest
    /// smaller image will be upscaled and be used instead.
    ///
    /// \param window the window to change.
    /// \param icon an SDL_Surface structure containing the icon for the window.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the window's position be set.
    ///
    /// If, at the time of this request, the window is in a fixed-size state such
    /// as maximized, this request may be deferred until the window returns to a
    /// resizable state.
    ///
    /// This can be used to reposition fullscreen-desktop windows onto a different
    /// display, however, exclusive fullscreen windows are locked to a specific
    /// display and can only be repositioned programmatically via
    /// SDL_SetWindowFullscreenMode().
    ///
    /// On some windowing systems this request is asynchronous and the new
    /// coordinates may not have have been applied immediately upon the return of
    /// this function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window position changes, an SDL_EVENT_WINDOW_MOVED event will be
    /// emitted with the window's new coordinates. Note that the new coordinates
    /// may not match the exact coordinates requested, as some windowing systems
    /// can restrict the position of the window in certain scenarios (e.g.
    /// constraining the position so the window is always within desktop bounds).
    /// Additionally, as this is just a request, it can be denied by the windowing
    /// system.
    ///
    /// \param window the window to reposition.
    /// \param x the x coordinate of the window, or `SDL_WINDOWPOS_CENTERED` or
    ///          `SDL_WINDOWPOS_UNDEFINED`.
    /// \param y the y coordinate of the window, or `SDL_WINDOWPOS_CENTERED` or
    ///          `SDL_WINDOWPOS_UNDEFINED`.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowPosition
    /// \sa SDL_SyncWindow
    pub fn SDL_SetWindowPosition(window: *mut SDL_Window, x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the position of a window.
    ///
    /// This is the current position of the window as last reported by the
    /// windowing system.
    ///
    /// If you do not need the value for one of the positions a NULL may be passed
    /// in the `x` or `y` parameter.
    ///
    /// \param window the window to query.
    /// \param x a pointer filled in with the x position of the window, may be
    ///          NULL.
    /// \param y a pointer filled in with the y position of the window, may be
    ///          NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowPosition
    pub fn SDL_GetWindowPosition(window: *mut SDL_Window, x: *mut ::core::ffi::c_int, y: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the size of a window's client area be set.
    ///
    /// If, at the time of this request, the window in a fixed-size state, such as
    /// maximized or fullscreen, the request will be deferred until the window
    /// exits this state and becomes resizable again.
    ///
    /// To change the fullscreen mode of a window, use
    /// SDL_SetWindowFullscreenMode()
    ///
    /// On some windowing systems, this request is asynchronous and the new window
    /// size may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window size changes, an SDL_EVENT_WINDOW_RESIZED event will be
    /// emitted with the new window dimensions. Note that the new dimensions may
    /// not match the exact size requested, as some windowing systems can restrict
    /// the window size in certain scenarios (e.g. constraining the size of the
    /// content area to remain within the usable desktop bounds). Additionally, as
    /// this is just a request, it can be denied by the windowing system.
    ///
    /// \param window the window to change.
    /// \param w the width of the window, must be > 0.
    /// \param h the height of the window, must be > 0.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSize
    /// \sa SDL_SetWindowFullscreenMode
    /// \sa SDL_SyncWindow
    pub fn SDL_SetWindowSize(window: *mut SDL_Window, w: ::core::ffi::c_int, h: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the size of a window's client area.
    ///
    /// The window pixel size may differ from its window coordinate size if the
    /// window is on a high pixel density display. Use SDL_GetWindowSizeInPixels()
    /// or SDL_GetRenderOutputSize() to get the real client area size in pixels.
    ///
    /// \param window the window to query the width and height from.
    /// \param w a pointer filled in with the width of the window, may be NULL.
    /// \param h a pointer filled in with the height of the window, may be NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetRenderOutputSize
    /// \sa SDL_GetWindowSizeInPixels
    /// \sa SDL_SetWindowSize
    pub fn SDL_GetWindowSize(window: *mut SDL_Window, w: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the safe area for this window.
    ///
    /// Some devices have portions of the screen which are partially obscured or
    /// not interactive, possibly due to on-screen controls, curved edges, camera
    /// notches, TV overscan, etc. This function provides the area of the window
    /// which is safe to have interactible content. You should continue rendering
    /// into the rest of the window, but it should not contain visually important
    /// or interactible content.
    ///
    /// \param window the window to query.
    /// \param rect a pointer filled in with the client area that is safe for
    ///             interactive content.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetWindowSafeArea(window: *mut SDL_Window, rect: *mut SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the aspect ratio of a window's client area be set.
    ///
    /// The aspect ratio is the ratio of width divided by height, e.g. 2560x1600
    /// would be 1.6. Larger aspect ratios are wider and smaller aspect ratios are
    /// narrower.
    ///
    /// If, at the time of this request, the window in a fixed-size state, such as
    /// maximized or fullscreen, the request will be deferred until the window
    /// exits this state and becomes resizable again.
    ///
    /// On some windowing systems, this request is asynchronous and the new window
    /// aspect ratio may not have have been applied immediately upon the return of
    /// this function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window size changes, an SDL_EVENT_WINDOW_RESIZED event will be
    /// emitted with the new window dimensions. Note that the new dimensions may
    /// not match the exact aspect ratio requested, as some windowing systems can
    /// restrict the window size in certain scenarios (e.g. constraining the size
    /// of the content area to remain within the usable desktop bounds).
    /// Additionally, as this is just a request, it can be denied by the windowing
    /// system.
    ///
    /// \param window the window to change.
    /// \param min_aspect the minimum aspect ratio of the window, or 0.0f for no
    ///                   limit.
    /// \param max_aspect the maximum aspect ratio of the window, or 0.0f for no
    ///                   limit.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowAspectRatio
    /// \sa SDL_SyncWindow
    pub fn SDL_SetWindowAspectRatio(window: *mut SDL_Window, min_aspect: ::core::ffi::c_float, max_aspect: ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the size of a window's client area.
    ///
    /// \param window the window to query the width and height from.
    /// \param min_aspect a pointer filled in with the minimum aspect ratio of the
    ///                   window, may be NULL.
    /// \param max_aspect a pointer filled in with the maximum aspect ratio of the
    ///                   window, may be NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowAspectRatio
    pub fn SDL_GetWindowAspectRatio(window: *mut SDL_Window, min_aspect: *mut ::core::ffi::c_float, max_aspect: *mut ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the size of a window's borders (decorations) around the client area.
    ///
    /// Note: If this function fails (returns -1), the size values will be
    /// initialized to 0, 0, 0, 0 (if a non-NULL pointer is provided), as if the
    /// window in question was borderless.
    ///
    /// Note: This function may fail on systems where the window has not yet been
    /// decorated by the display server (for example, immediately after calling
    /// SDL_CreateWindow). It is recommended that you wait at least until the
    /// window has been presented and composited, so that the window system has a
    /// chance to decorate the window and provide the border dimensions to SDL.
    ///
    /// This function also returns -1 if getting the information is not supported.
    ///
    /// \param window the window to query the size values of the border
    ///               (decorations) from.
    /// \param top pointer to variable for storing the size of the top border; NULL
    ///            is permitted.
    /// \param left pointer to variable for storing the size of the left border;
    ///             NULL is permitted.
    /// \param bottom pointer to variable for storing the size of the bottom
    ///               border; NULL is permitted.
    /// \param right pointer to variable for storing the size of the right border;
    ///              NULL is permitted.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSize
    pub fn SDL_GetWindowBordersSize(window: *mut SDL_Window, top: *mut ::core::ffi::c_int, left: *mut ::core::ffi::c_int, bottom: *mut ::core::ffi::c_int, right: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the size of a window's client area, in pixels.
    ///
    /// \param window the window from which the drawable size should be queried.
    /// \param w a pointer to variable for storing the width in pixels, may be
    ///          NULL.
    /// \param h a pointer to variable for storing the height in pixels, may be
    ///          NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateWindow
    /// \sa SDL_GetWindowSize
    pub fn SDL_GetWindowSizeInPixels(window: *mut SDL_Window, w: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the minimum size of a window's client area.
    ///
    /// \param window the window to change.
    /// \param min_w the minimum width of the window, or 0 for no limit.
    /// \param min_h the minimum height of the window, or 0 for no limit.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMinimumSize
    /// \sa SDL_SetWindowMaximumSize
    pub fn SDL_SetWindowMinimumSize(window: *mut SDL_Window, min_w: ::core::ffi::c_int, min_h: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the minimum size of a window's client area.
    ///
    /// \param window the window to query.
    /// \param w a pointer filled in with the minimum width of the window, may be
    ///          NULL.
    /// \param h a pointer filled in with the minimum height of the window, may be
    ///          NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMaximumSize
    /// \sa SDL_SetWindowMinimumSize
    pub fn SDL_GetWindowMinimumSize(window: *mut SDL_Window, w: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the maximum size of a window's client area.
    ///
    /// \param window the window to change.
    /// \param max_w the maximum width of the window, or 0 for no limit.
    /// \param max_h the maximum height of the window, or 0 for no limit.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMaximumSize
    /// \sa SDL_SetWindowMinimumSize
    pub fn SDL_SetWindowMaximumSize(window: *mut SDL_Window, max_w: ::core::ffi::c_int, max_h: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the maximum size of a window's client area.
    ///
    /// \param window the window to query.
    /// \param w a pointer filled in with the maximum width of the window, may be
    ///          NULL.
    /// \param h a pointer filled in with the maximum height of the window, may be
    ///          NULL.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMinimumSize
    /// \sa SDL_SetWindowMaximumSize
    pub fn SDL_GetWindowMaximumSize(window: *mut SDL_Window, w: *mut ::core::ffi::c_int, h: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the border state of a window.
    ///
    /// This will add or remove the window's `SDL_WINDOW_BORDERLESS` flag and add
    /// or remove the border from the actual window. This is a no-op if the
    /// window's border already matches the requested state.
    ///
    /// You can't change the border state of a fullscreen window.
    ///
    /// \param window the window of which to change the border state.
    /// \param bordered SDL_FALSE to remove border, SDL_TRUE to add border.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFlags
    pub fn SDL_SetWindowBordered(window: *mut SDL_Window, bordered: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the user-resizable state of a window.
    ///
    /// This will add or remove the window's `SDL_WINDOW_RESIZABLE` flag and
    /// allow/disallow user resizing of the window. This is a no-op if the window's
    /// resizable state already matches the requested state.
    ///
    /// You can't change the resizable state of a fullscreen window.
    ///
    /// \param window the window of which to change the resizable state.
    /// \param resizable SDL_TRUE to allow resizing, SDL_FALSE to disallow.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFlags
    pub fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the window to always be above the others.
    ///
    /// This will add or remove the window's `SDL_WINDOW_ALWAYS_ON_TOP` flag. This
    /// will bring the window to the front and keep the window above the rest.
    ///
    /// \param window the window of which to change the always on top state.
    /// \param on_top SDL_TRUE to set the window always on top, SDL_FALSE to
    ///               disable.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFlags
    pub fn SDL_SetWindowAlwaysOnTop(window: *mut SDL_Window, on_top: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Show a window.
    ///
    /// \param window the window to show.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_HideWindow
    /// \sa SDL_RaiseWindow
    pub fn SDL_ShowWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Hide a window.
    ///
    /// \param window the window to hide.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ShowWindow
    pub fn SDL_HideWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that a window be raised above other windows and gain the input
    /// focus.
    ///
    /// The result of this request is subject to desktop window manager policy,
    /// particularly if raising the requested window would result in stealing focus
    /// from another application. If the window is successfully raised and gains
    /// input focus, an SDL_EVENT_WINDOW_FOCUS_GAINED event will be emitted, and
    /// the window will have the SDL_WINDOW_INPUT_FOCUS flag set.
    ///
    /// \param window the window to raise.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_RaiseWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the window be made as large as possible.
    ///
    /// Non-resizable windows can't be maximized. The window must have the
    /// SDL_WINDOW_RESIZABLE flag set, or this will have no effect.
    ///
    /// On some windowing systems this request is asynchronous and the new window
    /// state may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window state changes, an SDL_EVENT_WINDOW_MAXIMIZED event will be
    /// emitted. Note that, as this is just a request, the windowing system can
    /// deny the state change.
    ///
    /// When maximizing a window, whether the constraints set via
    /// SDL_SetWindowMaximumSize() are honored depends on the policy of the window
    /// manager. Win32 and macOS enforce the constraints when maximizing, while X11
    /// and Wayland window managers may vary.
    ///
    /// \param window the window to maximize.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MinimizeWindow
    /// \sa SDL_RestoreWindow
    /// \sa SDL_SyncWindow
    pub fn SDL_MaximizeWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the window be minimized to an iconic representation.
    ///
    /// On some windowing systems this request is asynchronous and the new window
    /// state may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window state changes, an SDL_EVENT_WINDOW_MINIMIZED event will be
    /// emitted. Note that, as this is just a request, the windowing system can
    /// deny the state change.
    ///
    /// \param window the window to minimize.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MaximizeWindow
    /// \sa SDL_RestoreWindow
    /// \sa SDL_SyncWindow
    pub fn SDL_MinimizeWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the size and position of a minimized or maximized window be
    /// restored.
    ///
    /// On some windowing systems this request is asynchronous and the new window
    /// state may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call SDL_SyncWindow() to
    /// block until the changes have taken effect.
    ///
    /// When the window state changes, an SDL_EVENT_WINDOW_RESTORED event will be
    /// emitted. Note that, as this is just a request, the windowing system can
    /// deny the state change.
    ///
    /// \param window the window to restore.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_MaximizeWindow
    /// \sa SDL_MinimizeWindow
    /// \sa SDL_SyncWindow
    pub fn SDL_RestoreWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request that the window's fullscreen state be changed.
    ///
    /// By default a window in fullscreen state uses borderless fullscreen desktop
    /// mode, but a specific exclusive display mode can be set using
    /// SDL_SetWindowFullscreenMode().
    ///
    /// On some windowing systems this request is asynchronous and the new
    /// fullscreen state may not have have been applied immediately upon the return
    /// of this function. If an immediate change is required, call SDL_SyncWindow()
    /// to block until the changes have taken effect.
    ///
    /// When the window state changes, an SDL_EVENT_WINDOW_ENTER_FULLSCREEN or
    /// SDL_EVENT_WINDOW_LEAVE_FULLSCREEN event will be emitted. Note that, as this
    /// is just a request, it can be denied by the windowing system.
    ///
    /// \param window the window to change.
    /// \param fullscreen SDL_TRUE for fullscreen mode, SDL_FALSE for windowed
    ///                   mode.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowFullscreenMode
    /// \sa SDL_SetWindowFullscreenMode
    /// \sa SDL_SyncWindow
    pub fn SDL_SetWindowFullscreen(window: *mut SDL_Window, fullscreen: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Block until any pending window state is finalized.
    ///
    /// On asynchronous windowing systems, this acts as a synchronization barrier
    /// for pending window state. It will attempt to wait until any pending window
    /// state has been applied and is guaranteed to return within finite time. Note
    /// that for how long it can potentially block depends on the underlying window
    /// system, as window state changes may involve somewhat lengthy animations
    /// that must complete before the window is in its final requested state.
    ///
    /// On windowing systems where changes are immediate, this does nothing.
    ///
    /// \param window the window for which to wait for the pending state to be
    ///               applied.
    /// \returns SDL_TRUE on success or SDL_FALSE if the operation timed out before
    ///          the window was in the requested state.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowSize
    /// \sa SDL_SetWindowPosition
    /// \sa SDL_SetWindowFullscreen
    /// \sa SDL_MinimizeWindow
    /// \sa SDL_MaximizeWindow
    /// \sa SDL_RestoreWindow
    /// \sa SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS
    pub fn SDL_SyncWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Return whether the window has a surface associated with it.
    ///
    /// \param window the window to query.
    /// \returns SDL_TRUE if there is a surface associated with the window, or
    ///          SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSurface
    pub fn SDL_WindowHasSurface(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the SDL surface associated with the window.
    ///
    /// A new surface will be created with the optimal format for the window, if
    /// necessary. This surface will be freed when the window is destroyed. Do not
    /// free this surface.
    ///
    /// This surface will be invalidated if the window is resized. After resizing a
    /// window this function must be called again to return a valid surface.
    ///
    /// You may not combine this with 3D or the rendering API on this window.
    ///
    /// This function is affected by `SDL_HINT_FRAMEBUFFER_ACCELERATION`.
    ///
    /// \param window the window to query.
    /// \returns the surface associated with the window, or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DestroyWindowSurface
    /// \sa SDL_WindowHasSurface
    /// \sa SDL_UpdateWindowSurface
    /// \sa SDL_UpdateWindowSurfaceRects
    pub fn SDL_GetWindowSurface(window: *mut SDL_Window) -> *mut SDL_Surface;
}}

extern_sdlcall! {{
    /// Toggle VSync for the window surface.
    ///
    /// When a window surface is created, vsync defaults to
    /// SDL_WINDOW_SURFACE_VSYNC_DISABLED.
    ///
    /// The `vsync` parameter can be 1 to synchronize present with every vertical
    /// refresh, 2 to synchronize present with every second vertical refresh, etc.,
    /// SDL_WINDOW_SURFACE_VSYNC_ADAPTIVE for late swap tearing (adaptive vsync),
    /// or SDL_WINDOW_SURFACE_VSYNC_DISABLED to disable. Not every value is
    /// supported by every driver, so you should check the return value to see
    /// whether the requested setting is supported.
    ///
    /// \param window the window.
    /// \param vsync the vertical refresh sync interval.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSurfaceVSync
    pub fn SDL_SetWindowSurfaceVSync(window: *mut SDL_Window, vsync: ::core::ffi::c_int) -> SDL_bool;
}}

pub const SDL_WINDOW_SURFACE_VSYNC_DISABLED: ::core::primitive::i32 = 0;

pub const SDL_WINDOW_SURFACE_VSYNC_ADAPTIVE: ::core::primitive::i32 = -1;

extern_sdlcall! {{
    /// Get VSync for the window surface.
    ///
    /// \param window the window to query.
    /// \param vsync an int filled with the current vertical refresh sync interval.
    ///              See SDL_SetWindowSurfaceVSync() for the meaning of the value.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowSurfaceVSync
    pub fn SDL_GetWindowSurfaceVSync(window: *mut SDL_Window, vsync: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Copy the window surface to the screen.
    ///
    /// This is the function you use to reflect any changes to the surface on the
    /// screen.
    ///
    /// This function is equivalent to the SDL 1.2 API SDL_Flip().
    ///
    /// \param window the window to update.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSurface
    /// \sa SDL_UpdateWindowSurfaceRects
    pub fn SDL_UpdateWindowSurface(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Copy areas of the window surface to the screen.
    ///
    /// This is the function you use to reflect changes to portions of the surface
    /// on the screen.
    ///
    /// This function is equivalent to the SDL 1.2 API SDL_UpdateRects().
    ///
    /// Note that this function will update _at least_ the rectangles specified,
    /// but this is only intended as an optimization; in practice, this might
    /// update more of the screen (or all of the screen!), depending on what method
    /// SDL uses to send pixels to the system.
    ///
    /// \param window the window to update.
    /// \param rects an array of SDL_Rect structures representing areas of the
    ///              surface to copy, in pixels.
    /// \param numrects the number of rectangles.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSurface
    /// \sa SDL_UpdateWindowSurface
    pub fn SDL_UpdateWindowSurfaceRects(window: *mut SDL_Window, rects: *const SDL_Rect, numrects: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Destroy the surface associated with the window.
    ///
    /// \param window the window to update.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowSurface
    /// \sa SDL_WindowHasSurface
    pub fn SDL_DestroyWindowSurface(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set a window's keyboard grab mode.
    ///
    /// Keyboard grab enables capture of system keyboard shortcuts like Alt+Tab or
    /// the Meta/Super key. Note that not all system keyboard shortcuts can be
    /// captured by applications (one example is Ctrl+Alt+Del on Windows).
    ///
    /// This is primarily intended for specialized applications such as VNC clients
    /// or VM frontends. Normal games should not use keyboard grab.
    ///
    /// When keyboard grab is enabled, SDL will continue to handle Alt+Tab when the
    /// window is full-screen to ensure the user is not trapped in your
    /// application. If you have a custom keyboard shortcut to exit fullscreen
    /// mode, you may suppress this behavior with
    /// `SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED`.
    ///
    /// If the caller enables a grab while another window is currently grabbed, the
    /// other window loses its grab in favor of the caller's window.
    ///
    /// \param window the window for which the keyboard grab mode should be set.
    /// \param grabbed this is SDL_TRUE to grab keyboard, and SDL_FALSE to release.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowKeyboardGrab
    /// \sa SDL_SetWindowMouseGrab
    pub fn SDL_SetWindowKeyboardGrab(window: *mut SDL_Window, grabbed: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set a window's mouse grab mode.
    ///
    /// Mouse grab confines the mouse cursor to the window.
    ///
    /// \param window the window for which the mouse grab mode should be set.
    /// \param grabbed this is SDL_TRUE to grab mouse, and SDL_FALSE to release.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMouseGrab
    /// \sa SDL_SetWindowKeyboardGrab
    pub fn SDL_SetWindowMouseGrab(window: *mut SDL_Window, grabbed: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get a window's keyboard grab mode.
    ///
    /// \param window the window to query.
    /// \returns SDL_TRUE if keyboard is grabbed, and SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowKeyboardGrab
    pub fn SDL_GetWindowKeyboardGrab(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get a window's mouse grab mode.
    ///
    /// \param window the window to query.
    /// \returns SDL_TRUE if mouse is grabbed, and SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowKeyboardGrab
    pub fn SDL_GetWindowMouseGrab(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the window that currently has an input grab enabled.
    ///
    /// \returns the window if input is grabbed or NULL otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowMouseGrab
    /// \sa SDL_SetWindowKeyboardGrab
    pub fn SDL_GetGrabbedWindow() -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Confines the cursor to the specified area of a window.
    ///
    /// Note that this does NOT grab the cursor, it only defines the area a cursor
    /// is restricted to when the window has mouse focus.
    ///
    /// \param window the window that will be associated with the barrier.
    /// \param rect a rectangle area in window-relative coordinates. If NULL the
    ///             barrier for the specified window will be destroyed.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowMouseRect
    /// \sa SDL_SetWindowMouseGrab
    pub fn SDL_SetWindowMouseRect(window: *mut SDL_Window, rect: *const SDL_Rect) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the mouse confinement rectangle of a window.
    ///
    /// \param window the window to query.
    /// \returns a pointer to the mouse confinement rectangle of a window, or NULL
    ///          if there isn't one.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowMouseRect
    pub fn SDL_GetWindowMouseRect(window: *mut SDL_Window) -> *const SDL_Rect;
}}

extern_sdlcall! {{
    /// Set the opacity for a window.
    ///
    /// The parameter `opacity` will be clamped internally between 0.0f
    /// (transparent) and 1.0f (opaque).
    ///
    /// This function also returns -1 if setting the opacity isn't supported.
    ///
    /// \param window the window which will be made transparent or opaque.
    /// \param opacity the opacity value (0.0f - transparent, 1.0f - opaque).
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetWindowOpacity
    pub fn SDL_SetWindowOpacity(window: *mut SDL_Window, opacity: ::core::ffi::c_float) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the opacity of a window.
    ///
    /// If transparency isn't supported on this platform, opacity will be returned
    /// as 1.0f without error.
    ///
    /// \param window the window to get the current opacity value from.
    /// \returns the opacity, (0.0f - transparent, 1.0f - opaque), or -1.0f on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetWindowOpacity
    pub fn SDL_GetWindowOpacity(window: *mut SDL_Window) -> ::core::ffi::c_float;
}}

extern_sdlcall! {{
    /// Set the window as a modal to a parent window.
    ///
    /// If the window is already modal to an existing window, it will be reparented
    /// to the new owner. Setting the parent window to null unparents the modal
    /// window and removes modal status.
    ///
    /// Setting a window as modal to a parent that is a descendent of the modal
    /// window results in undefined behavior.
    ///
    /// \param modal_window the window that should be set modal.
    /// \param parent_window the parent window for the modal window.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetWindowModalFor(modal_window: *mut SDL_Window, parent_window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set whether the window may have input focus.
    ///
    /// \param window the window to set focusable state.
    /// \param focusable SDL_TRUE to allow input focus, SDL_FALSE to not allow
    ///                  input focus.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetWindowFocusable(window: *mut SDL_Window, focusable: SDL_bool) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Display the system-level window menu.
    ///
    /// This default window menu is provided by the system and on some platforms
    /// provides functionality for setting or changing privileged state on the
    /// window, such as moving it between workspaces or displays, or toggling the
    /// always-on-top property.
    ///
    /// On platforms or desktops where this is unsupported, this function does
    /// nothing.
    ///
    /// \param window the window for which the menu will be displayed.
    /// \param x the x coordinate of the menu, relative to the origin (top-left) of
    ///          the client area.
    /// \param y the y coordinate of the menu, relative to the origin (top-left) of
    ///          the client area.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ShowWindowSystemMenu(window: *mut SDL_Window, x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> SDL_bool;
}}

/// Possible return values from the SDL_HitTest callback.
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_HitTest
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_HITTEST_NORMAL`], [`SDL_HITTEST_DRAGGABLE`], [`SDL_HITTEST_RESIZE_TOPLEFT`], [`SDL_HITTEST_RESIZE_TOP`], [`SDL_HITTEST_RESIZE_TOPRIGHT`], [`SDL_HITTEST_RESIZE_RIGHT`], [`SDL_HITTEST_RESIZE_BOTTOMRIGHT`], [`SDL_HITTEST_RESIZE_BOTTOM`], [`SDL_HITTEST_RESIZE_BOTTOMLEFT`], [`SDL_HITTEST_RESIZE_LEFT`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_HitTestResult(pub ::core::ffi::c_int);
impl SDL_HitTestResult {
    /// Region is normal. No special properties.
    pub const NORMAL: Self = Self(0);
    /// Region can drag entire window.
    pub const DRAGGABLE: Self = Self(1);
    /// Region is the resizable top-left corner border.
    pub const RESIZE_TOPLEFT: Self = Self(2);
    /// Region is the resizable top border.
    pub const RESIZE_TOP: Self = Self(3);
    /// Region is the resizable top-right corner border.
    pub const RESIZE_TOPRIGHT: Self = Self(4);
    /// Region is the resizable right border.
    pub const RESIZE_RIGHT: Self = Self(5);
    /// Region is the resizable bottom-right corner border.
    pub const RESIZE_BOTTOMRIGHT: Self = Self(6);
    /// Region is the resizable bottom border.
    pub const RESIZE_BOTTOM: Self = Self(7);
    /// Region is the resizable bottom-left corner border.
    pub const RESIZE_BOTTOMLEFT: Self = Self(8);
    /// Region is the resizable left border.
    pub const RESIZE_LEFT: Self = Self(9);
}
/// Region is normal. No special properties.
pub const SDL_HITTEST_NORMAL: SDL_HitTestResult = SDL_HitTestResult::NORMAL;
/// Region can drag entire window.
pub const SDL_HITTEST_DRAGGABLE: SDL_HitTestResult = SDL_HitTestResult::DRAGGABLE;
/// Region is the resizable top-left corner border.
pub const SDL_HITTEST_RESIZE_TOPLEFT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_TOPLEFT;
/// Region is the resizable top border.
pub const SDL_HITTEST_RESIZE_TOP: SDL_HitTestResult = SDL_HitTestResult::RESIZE_TOP;
/// Region is the resizable top-right corner border.
pub const SDL_HITTEST_RESIZE_TOPRIGHT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_TOPRIGHT;
/// Region is the resizable right border.
pub const SDL_HITTEST_RESIZE_RIGHT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_RIGHT;
/// Region is the resizable bottom-right corner border.
pub const SDL_HITTEST_RESIZE_BOTTOMRIGHT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_BOTTOMRIGHT;
/// Region is the resizable bottom border.
pub const SDL_HITTEST_RESIZE_BOTTOM: SDL_HitTestResult = SDL_HitTestResult::RESIZE_BOTTOM;
/// Region is the resizable bottom-left corner border.
pub const SDL_HITTEST_RESIZE_BOTTOMLEFT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_BOTTOMLEFT;
/// Region is the resizable left border.
pub const SDL_HITTEST_RESIZE_LEFT: SDL_HitTestResult = SDL_HitTestResult::RESIZE_LEFT;

/// Callback used for hit-testing.
///
/// \param win the SDL_Window where hit-testing was set on.
/// \param area an SDL_Point which should be hit-tested.
/// \param data what was passed as `callback_data` to SDL_SetWindowHitTest().
/// \returns an SDL_HitTestResult value.
///
/// \sa SDL_SetWindowHitTest
pub type SDL_HitTest = ::core::option::Option<extern_sdlcall!(fn(win: *mut SDL_Window, area: *const SDL_Point, data: *mut ::core::ffi::c_void) -> SDL_HitTestResult)>;

extern_sdlcall! {{
    /// Provide a callback that decides if a window region has special properties.
    ///
    /// Normally windows are dragged and resized by decorations provided by the
    /// system window manager (a title bar, borders, etc), but for some apps, it
    /// makes sense to drag them from somewhere else inside the window itself; for
    /// example, one might have a borderless window that wants to be draggable from
    /// any part, or simulate its own title bar, etc.
    ///
    /// This function lets the app provide a callback that designates pieces of a
    /// given window as special. This callback is run during event processing if we
    /// need to tell the OS to treat a region of the window specially; the use of
    /// this callback is known as "hit testing."
    ///
    /// Mouse input may not be delivered to your application if it is within a
    /// special area; the OS will often apply that input to moving the window or
    /// resizing the window and not deliver it to the application.
    ///
    /// Specifying NULL for a callback disables hit-testing. Hit-testing is
    /// disabled by default.
    ///
    /// Platforms that don't support this functionality will return SDL_FALSE
    /// unconditionally, even if you're attempting to disable hit-testing.
    ///
    /// Your callback may fire at any time, and its firing does not indicate any
    /// specific behavior (for example, on Windows, this certainly might fire when
    /// the OS is deciding whether to drag your window, but it fires for lots of
    /// other reasons, too, some unrelated to anything you probably care about _and
    /// when the mouse isn't actually at the location it is testing_). Since this
    /// can fire at any time, you should try to keep your callback efficient,
    /// devoid of allocations, etc.
    ///
    /// \param window the window to set hit-testing on.
    /// \param callback the function to call when doing a hit-test.
    /// \param callback_data an app-defined void pointer passed to **callback**.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetWindowHitTest(window: *mut SDL_Window, callback: SDL_HitTest, callback_data: *mut ::core::ffi::c_void) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Set the shape of a transparent window.
    ///
    /// This sets the alpha channel of a transparent window and any fully
    /// transparent areas are also transparent to mouse clicks. If you are using
    /// something besides the SDL render API, then you are responsible for setting
    /// the alpha channel of the window yourself.
    ///
    /// The shape is copied inside this function, so you can free it afterwards. If
    /// your shape surface changes, you should call SDL_SetWindowShape() again to
    /// update the window.
    ///
    /// The window must have been created with the SDL_WINDOW_TRANSPARENT flag.
    ///
    /// \param window the window.
    /// \param shape the surface representing the shape of the window, or NULL to
    ///              remove any current shape.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_SetWindowShape(window: *mut SDL_Window, shape: *mut SDL_Surface) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Request a window to demand attention from the user.
    ///
    /// \param window the window to be flashed.
    /// \param operation the operation to perform.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_FlashWindow(window: *mut SDL_Window, operation: SDL_FlashOperation) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Destroy a window.
    ///
    /// Any popups or modal windows owned by the window will be recursively
    /// destroyed as well.
    ///
    /// \param window the window to destroy.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreatePopupWindow
    /// \sa SDL_CreateWindow
    /// \sa SDL_CreateWindowWithProperties
    pub fn SDL_DestroyWindow(window: *mut SDL_Window);
}}

extern_sdlcall! {{
    /// Check whether the screensaver is currently enabled.
    ///
    /// The screensaver is disabled by default.
    ///
    /// The default can also be changed using `SDL_HINT_VIDEO_ALLOW_SCREENSAVER`.
    ///
    /// \returns SDL_TRUE if the screensaver is enabled, SDL_FALSE if it is
    ///          disabled.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DisableScreenSaver
    /// \sa SDL_EnableScreenSaver
    pub fn SDL_ScreenSaverEnabled() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Allow the screen to be blanked by a screen saver.
    ///
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DisableScreenSaver
    /// \sa SDL_ScreenSaverEnabled
    pub fn SDL_EnableScreenSaver() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Prevent the screen from being blanked by a screen saver.
    ///
    /// If you disable the screensaver, it is automatically re-enabled when SDL
    /// quits.
    ///
    /// The screensaver is disabled by default, but this may by changed by
    /// SDL_HINT_VIDEO_ALLOW_SCREENSAVER.
    ///
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_EnableScreenSaver
    /// \sa SDL_ScreenSaverEnabled
    pub fn SDL_DisableScreenSaver() -> SDL_bool;
}}

extern_sdlcall! {{
    /// Dynamically load an OpenGL library.
    ///
    /// This should be done after initializing the video driver, but before
    /// creating any OpenGL windows. If no OpenGL library is loaded, the default
    /// library will be loaded upon creation of the first OpenGL window.
    ///
    /// If you do this, you need to retrieve all of the GL functions used in your
    /// program from the dynamic library using SDL_GL_GetProcAddress().
    ///
    /// \param path the platform dependent OpenGL library name, or NULL to open the
    ///             default OpenGL library.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_GetProcAddress
    /// \sa SDL_GL_UnloadLibrary
    pub fn SDL_GL_LoadLibrary(path: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get an OpenGL function by name.
    ///
    /// If the GL library is loaded at runtime with SDL_GL_LoadLibrary(), then all
    /// GL functions must be retrieved this way. Usually this is used to retrieve
    /// function pointers to OpenGL extensions.
    ///
    /// There are some quirks to looking up OpenGL functions that require some
    /// extra care from the application. If you code carefully, you can handle
    /// these quirks without any platform-specific code, though:
    ///
    /// - On Windows, function pointers are specific to the current GL context;
    ///   this means you need to have created a GL context and made it current
    ///   before calling SDL_GL_GetProcAddress(). If you recreate your context or
    ///   create a second context, you should assume that any existing function
    ///   pointers aren't valid to use with it. This is (currently) a
    ///   Windows-specific limitation, and in practice lots of drivers don't suffer
    ///   this limitation, but it is still the way the wgl API is documented to
    ///   work and you should expect crashes if you don't respect it. Store a copy
    ///   of the function pointers that comes and goes with context lifespan.
    /// - On X11, function pointers returned by this function are valid for any
    ///   context, and can even be looked up before a context is created at all.
    ///   This means that, for at least some common OpenGL implementations, if you
    ///   look up a function that doesn't exist, you'll get a non-NULL result that
    ///   is _NOT_ safe to call. You must always make sure the function is actually
    ///   available for a given GL context before calling it, by checking for the
    ///   existence of the appropriate extension with SDL_GL_ExtensionSupported(),
    ///   or verifying that the version of OpenGL you're using offers the function
    ///   as core functionality.
    /// - Some OpenGL drivers, on all platforms, *will* return NULL if a function
    ///   isn't supported, but you can't count on this behavior. Check for
    ///   extensions you use, and if you get a NULL anyway, act as if that
    ///   extension wasn't available. This is probably a bug in the driver, but you
    ///   can code defensively for this scenario anyhow.
    /// - Just because you're on Linux/Unix, don't assume you'll be using X11.
    ///   Next-gen display servers are waiting to replace it, and may or may not
    ///   make the same promises about function pointers.
    /// - OpenGL function pointers must be declared `APIENTRY` as in the example
    ///   code. This will ensure the proper calling convention is followed on
    ///   platforms where this matters (Win32) thereby avoiding stack corruption.
    ///
    /// \param proc the name of an OpenGL function.
    /// \returns a pointer to the named OpenGL function. The returned pointer
    ///          should be cast to the appropriate function signature.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_ExtensionSupported
    /// \sa SDL_GL_LoadLibrary
    /// \sa SDL_GL_UnloadLibrary
    pub fn SDL_GL_GetProcAddress(proc: *const ::core::ffi::c_char) -> SDL_FunctionPointer;
}}

extern_sdlcall! {{
    /// Get an EGL library function by name.
    ///
    /// If an EGL library is loaded, this function allows applications to get entry
    /// points for EGL functions. This is useful to provide to an EGL API and
    /// extension loader.
    ///
    /// \param proc the name of an EGL function.
    /// \returns a pointer to the named EGL function. The returned pointer should
    ///          be cast to the appropriate function signature.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_EGL_GetCurrentDisplay
    pub fn SDL_EGL_GetProcAddress(proc: *const ::core::ffi::c_char) -> SDL_FunctionPointer;
}}

extern_sdlcall! {{
    /// Unload the OpenGL library previously loaded by SDL_GL_LoadLibrary().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_LoadLibrary
    pub fn SDL_GL_UnloadLibrary();
}}

extern_sdlcall! {{
    /// Check if an OpenGL extension is supported for the current context.
    ///
    /// This function operates on the current GL context; you must have created a
    /// context and it must be current before calling this function. Do not assume
    /// that all contexts you create will have the same set of extensions
    /// available, or that recreating an existing context will offer the same
    /// extensions again.
    ///
    /// While it's probably not a massive overhead, this function is not an O(1)
    /// operation. Check the extensions you care about after creating the GL
    /// context and save that information somewhere instead of calling the function
    /// every time you need to know.
    ///
    /// \param extension the name of the extension to check.
    /// \returns SDL_TRUE if the extension is supported, SDL_FALSE otherwise.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GL_ExtensionSupported(extension: *const ::core::ffi::c_char) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Reset all previously set OpenGL context attributes to their default values.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_GetAttribute
    /// \sa SDL_GL_SetAttribute
    pub fn SDL_GL_ResetAttributes();
}}

extern_sdlcall! {{
    /// Set an OpenGL window attribute before window creation.
    ///
    /// This function sets the OpenGL attribute `attr` to `value`. The requested
    /// attributes should be set before creating an OpenGL window. You should use
    /// SDL_GL_GetAttribute() to check the values after creating the OpenGL
    /// context, since the values obtained can differ from the requested ones.
    ///
    /// \param attr an SDL_GLattr enum value specifying the OpenGL attribute to
    ///             set.
    /// \param value the desired value for the attribute.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_GetAttribute
    /// \sa SDL_GL_ResetAttributes
    pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the actual value for an attribute from the current context.
    ///
    /// \param attr an SDL_GLattr enum value specifying the OpenGL attribute to
    ///             get.
    /// \param value a pointer filled in with the current value of `attr`.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_ResetAttributes
    /// \sa SDL_GL_SetAttribute
    pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Create an OpenGL context for an OpenGL window, and make it current.
    ///
    /// Windows users new to OpenGL should note that, for historical reasons, GL
    /// functions added after OpenGL version 1.1 are not available by default.
    /// Those functions must be loaded at run-time, either with an OpenGL
    /// extension-handling library or with SDL_GL_GetProcAddress() and its related
    /// functions.
    ///
    /// SDL_GLContext is opaque to the application.
    ///
    /// \param window the window to associate with the context.
    /// \returns the OpenGL context associated with `window` or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_DestroyContext
    /// \sa SDL_GL_MakeCurrent
    pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
}}

extern_sdlcall! {{
    /// Set up an OpenGL context for rendering into an OpenGL window.
    ///
    /// The context must have been created with a compatible window.
    ///
    /// \param window the window to associate with the context.
    /// \param context the OpenGL context to associate with the window.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_CreateContext
    pub fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: SDL_GLContext) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the currently active OpenGL window.
    ///
    /// \returns the currently active OpenGL window on success or NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;
}}

extern_sdlcall! {{
    /// Get the currently active OpenGL context.
    ///
    /// \returns the currently active OpenGL context or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_MakeCurrent
    pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
}}

extern_sdlcall! {{
    /// Get the currently active EGL display.
    ///
    /// \returns the currently active EGL display or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EGL_GetCurrentDisplay() -> SDL_EGLDisplay;
}}

extern_sdlcall! {{
    /// Get the currently active EGL config.
    ///
    /// \returns the currently active EGL config or NULL on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EGL_GetCurrentConfig() -> SDL_EGLConfig;
}}

extern_sdlcall! {{
    /// Get the EGL surface associated with the window.
    ///
    /// \param window the window to query.
    /// \returns the EGLSurface pointer associated with the window, or NULL on
    ///          failure.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EGL_GetWindowSurface(window: *mut SDL_Window) -> SDL_EGLSurface;
}}

extern_sdlcall! {{
    /// Sets the callbacks for defining custom EGLAttrib arrays for EGL
    /// initialization.
    ///
    /// Each callback should return a pointer to an EGL attribute array terminated
    /// with EGL_NONE. Callbacks may return NULL pointers to signal an error, which
    /// will cause the SDL_CreateWindow process to fail gracefully.
    ///
    /// The arrays returned by each callback will be appended to the existing
    /// attribute arrays defined by SDL.
    ///
    /// NOTE: These callback pointers will be reset after SDL_GL_ResetAttributes.
    ///
    /// \param platformAttribCallback callback for attributes to pass to
    ///                               eglGetPlatformDisplay.
    /// \param surfaceAttribCallback callback for attributes to pass to
    ///                              eglCreateSurface.
    /// \param contextAttribCallback callback for attributes to pass to
    ///                              eglCreateContext.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_EGL_SetAttributeCallbacks(platformAttribCallback: SDL_EGLAttribArrayCallback, surfaceAttribCallback: SDL_EGLIntArrayCallback, contextAttribCallback: SDL_EGLIntArrayCallback);
}}

extern_sdlcall! {{
    /// Set the swap interval for the current OpenGL context.
    ///
    /// Some systems allow specifying -1 for the interval, to enable adaptive
    /// vsync. Adaptive vsync works the same as vsync, but if you've already missed
    /// the vertical retrace for a given frame, it swaps buffers immediately, which
    /// might be less jarring for the user during occasional framerate drops. If an
    /// application requests adaptive vsync and the system does not support it,
    /// this function will fail and return SDL_FALSE. In such a case, you should
    /// probably retry the call with 1 for the interval.
    ///
    /// Adaptive vsync is implemented for some glX drivers with
    /// GLX_EXT_swap_control_tear, and for some Windows drivers with
    /// WGL_EXT_swap_control_tear.
    ///
    /// Read more on the Khronos wiki:
    /// https://www.khronos.org/opengl/wiki/Swap_Interval#Adaptive_Vsync
    ///
    /// \param interval 0 for immediate updates, 1 for updates synchronized with
    ///                 the vertical retrace, -1 for adaptive vsync.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_GetSwapInterval
    pub fn SDL_GL_SetSwapInterval(interval: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Get the swap interval for the current OpenGL context.
    ///
    /// If the system can't determine the swap interval, or there isn't a valid
    /// current context, this function will set *interval to 0 as a safe default.
    ///
    /// \param interval output interval value. 0 if there is no vertical retrace
    ///                 synchronization, 1 if the buffer swap is synchronized with
    ///                 the vertical retrace, and -1 if late swaps happen
    ///                 immediately instead of waiting for the next retrace.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_SetSwapInterval
    pub fn SDL_GL_GetSwapInterval(interval: *mut ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Update a window with OpenGL rendering.
    ///
    /// This is used with double-buffered OpenGL contexts, which are the default.
    ///
    /// On macOS, make sure you bind 0 to the draw framebuffer before swapping the
    /// window, otherwise nothing will happen. If you aren't using
    /// glBindFramebuffer(), this is the default and you won't have to do anything
    /// extra.
    ///
    /// \param window the window to change.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GL_SwapWindow(window: *mut SDL_Window) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Delete an OpenGL context.
    ///
    /// \param context the OpenGL context to be deleted.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GL_CreateContext
    pub fn SDL_GL_DestroyContext(context: SDL_GLContext) -> SDL_bool;
}}

#[repr(C)]
#[non_exhaustive]
pub struct SDL_DisplayModeData { _opaque: [::core::primitive::u8; 0] }

#[repr(C)]
#[non_exhaustive]
pub struct SDL_GLContextState { _opaque: [::core::primitive::u8; 0] }

/// The struct used as an opaque handle to a window.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_CreateWindow
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Window { _opaque: [::core::primitive::u8; 0] }
