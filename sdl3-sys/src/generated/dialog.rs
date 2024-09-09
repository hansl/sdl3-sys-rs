#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens, clippy::too_long_first_doc_paragraph, clippy::unnecessary_cast)]

//! # CategoryDialog
//!
//! File dialog support.

use super::stdinc::*;

use super::error::*;

use super::video::*;

/// An entry for filters for file dialogs.
///
/// `name` is a user-readable label for the filter (for example, "Office
/// document").
///
/// `pattern` is a semicolon-separated list of file extensions (for example,
/// "doc;docx"). File extensions may only contain alphanumeric characters,
/// hyphens, underscores and periods. Alternatively, the whole string can be a
/// single asterisk ("*"), which serves as an "All files" filter.
///
/// \since This struct is available since SDL 3.0.0.
///
/// \sa SDL_DialogFileCallback
/// \sa SDL_ShowOpenFileDialog
/// \sa SDL_ShowSaveFileDialog
/// \sa SDL_ShowOpenFolderDialog
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_DialogFileFilter {
    pub name: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
}

/// Callback used by file dialog functions.
///
/// The specific usage is described in each function.
///
/// If `filelist` is:
///
/// - NULL, an error occurred. Details can be obtained with SDL_GetError().
/// - A pointer to NULL, the user either didn't choose any file or canceled the
///   dialog.
/// - A pointer to non-`NULL`, the user chose one or more files. The argument
///   is a null-terminated list of pointers to C strings, each containing a
///   path.
///
/// The filelist argument does not need to be freed; it will automatically be
/// freed when the callback returns.
///
/// The filter argument is the index of the filter that was selected, or -1 if
/// no filter was selected or if the platform or method doesn't support
/// fetching the selected filter.
///
/// \param userdata an app-provided pointer, for the callback's use.
/// \param filelist the file(s) chosen by the user.
/// \param filter index of the selected filter.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_DialogFileFilter
/// \sa SDL_ShowOpenFileDialog
/// \sa SDL_ShowSaveFileDialog
/// \sa SDL_ShowOpenFolderDialog
pub type SDL_DialogFileCallback = ::core::option::Option<extern "C" fn(userdata: *mut ::core::ffi::c_void, filelist: *const *const ::core::ffi::c_char, filter: ::core::ffi::c_int)>;

extern "C" {
    /// Displays a dialog that lets the user select a file on their filesystem.
    ///
    /// This function should only be invoked from the main thread.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// Depending on the platform, the user may be allowed to input paths that
    /// don't yet exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to SDL_PumpEvents in their main loop.
    ///
    /// \param callback an SDL_DialogFileCallback to be invoked when the user
    ///                 selects a file and accepts, or cancels the dialog, or an
    ///                 error occurs. The first argument is a null-terminated list
    ///                 of C strings, representing the paths chosen by the user.
    ///                 The list will be empty if the user canceled the dialog, and
    ///                 it will be NULL if an error occurred. If an error occurred,
    ///                 it can be fetched with SDL_GetError(). The second argument
    ///                 is the userdata pointer passed to the function. The third
    ///                 argument is the index of the filter selected by the user,
    ///                 or one past the index of the last filter (therefore the
    ///                 index of the terminating NULL filter) if no filter was
    ///                 chosen, or -1 if the platform does not support detecting
    ///                 the selected filter.
    /// \param userdata an optional pointer to pass extra data to the callback when
    ///                 it will be invoked.
    /// \param window the window that the dialog should be modal for. May be NULL.
    ///               Not all platforms support this option.
    /// \param filters a list of SDL_DialogFileFilter's. May be NULL. Not all
    ///                platforms support this option, and platforms that do support
    ///                it may allow the user to ignore the filters.
    /// \param nfilters the number of filters. Ignored if filters is NULL.
    /// \param default_location the default folder or file to start the dialog at.
    ///                         May be NULL. Not all platforms support this option.
    /// \param allow_many if non-zero, the user will be allowed to select multiple
    ///                   entries. Not all platforms support this option.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DialogFileCallback
    /// \sa SDL_DialogFileFilter
    /// \sa SDL_ShowSaveFileDialog
    /// \sa SDL_ShowOpenFolderDialog
    pub fn SDL_ShowOpenFileDialog(callback: SDL_DialogFileCallback, userdata: *mut ::core::ffi::c_void, window: *mut SDL_Window, filters: *const SDL_DialogFileFilter, nfilters: ::core::ffi::c_int, default_location: *const ::core::ffi::c_char, allow_many: SDL_bool);
}

extern "C" {
    /// Displays a dialog that lets the user choose a new or existing file on their
    /// filesystem.
    ///
    /// This function should only be invoked from the main thread.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// The chosen file may or may not already exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to SDL_PumpEvents in their main loop.
    ///
    /// \param callback an SDL_DialogFileCallback to be invoked when the user
    ///                 selects a file and accepts, or cancels the dialog, or an
    ///                 error occurs. The first argument is a null-terminated list
    ///                 of C strings, representing the paths chosen by the user.
    ///                 The list will be empty if the user canceled the dialog, and
    ///                 it will be NULL if an error occurred. If an error occurred,
    ///                 it can be fetched with SDL_GetError(). The second argument
    ///                 is the userdata pointer passed to the function. The third
    ///                 argument is the index of the filter selected by the user,
    ///                 or one past the index of the last filter (therefore the
    ///                 index of the terminating NULL filter) if no filter was
    ///                 chosen, or -1 if the platform does not support detecting
    ///                 the selected filter.
    /// \param userdata an optional pointer to pass extra data to the callback when
    ///                 it will be invoked.
    /// \param window the window that the dialog should be modal for. May be NULL.
    ///               Not all platforms support this option.
    /// \param filters a list of SDL_DialogFileFilter's. May be NULL. Not all
    ///                platforms support this option, and platforms that do support
    ///                it may allow the user to ignore the filters.
    /// \param nfilters the number of filters. Ignored if filters is NULL.
    /// \param default_location the default folder or file to start the dialog at.
    ///                         May be NULL. Not all platforms support this option.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DialogFileCallback
    /// \sa SDL_DialogFileFilter
    /// \sa SDL_ShowOpenFileDialog
    /// \sa SDL_ShowOpenFolderDialog
    pub fn SDL_ShowSaveFileDialog(callback: SDL_DialogFileCallback, userdata: *mut ::core::ffi::c_void, window: *mut SDL_Window, filters: *const SDL_DialogFileFilter, nfilters: ::core::ffi::c_int, default_location: *const ::core::ffi::c_char);
}

extern "C" {
    /// Displays a dialog that lets the user select a folder on their filesystem.
    ///
    /// This function should only be invoked from the main thread.
    ///
    /// This is an asynchronous function; it will return immediately, and the
    /// result will be passed to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user
    /// chose. The list will be empty if the user canceled the dialog, and it will
    /// be NULL if an error occurred.
    ///
    /// Note that the callback may be called from a different thread than the one
    /// the function was invoked on.
    ///
    /// Depending on the platform, the user may be allowed to input paths that
    /// don't yet exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which
    /// requires an event-handling loop. Apps that do not use SDL to handle events
    /// should add a call to SDL_PumpEvents in their main loop.
    ///
    /// \param callback an SDL_DialogFileCallback to be invoked when the user
    ///                 selects a file and accepts, or cancels the dialog, or an
    ///                 error occurs. The first argument is a null-terminated list
    ///                 of C strings, representing the paths chosen by the user.
    ///                 The list will be empty if the user canceled the dialog, and
    ///                 it will be NULL if an error occurred. If an error occurred,
    ///                 it can be fetched with SDL_GetError(). The second argument
    ///                 is the userdata pointer passed to the function. The third
    ///                 argument is always -1 for SDL_ShowOpenFolderDialog.
    /// \param userdata an optional pointer to pass extra data to the callback when
    ///                 it will be invoked.
    /// \param window the window that the dialog should be modal for. May be NULL.
    ///               Not all platforms support this option.
    /// \param default_location the default folder or file to start the dialog at.
    ///                         May be NULL. Not all platforms support this option.
    /// \param allow_many if non-zero, the user will be allowed to select multiple
    ///                   entries. Not all platforms support this option.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_DialogFileCallback
    /// \sa SDL_ShowOpenFileDialog
    /// \sa SDL_ShowSaveFileDialog
    pub fn SDL_ShowOpenFolderDialog(callback: SDL_DialogFileCallback, userdata: *mut ::core::ffi::c_void, window: *mut SDL_Window, default_location: *const ::core::ffi::c_char, allow_many: SDL_bool);
}

