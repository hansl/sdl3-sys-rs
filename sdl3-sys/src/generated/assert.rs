//! A helpful assertion macro!
//!
//! SDL assertions operate like your usual `assert` macro, but with some added
//! features:
//!
//! - It uses a trick with the `sizeof` operator, so disabled assertions
//!   vaporize out of the compiled code, but variables only referenced in the
//!   assertion won't trigger compiler warnings about being unused.
//! - It is safe to use with a dangling-else: `if (x) SDL_assert(y); else
//!   do_something();`
//! - It works the same everywhere, instead of counting on various platforms'
//!   compiler and C runtime to behave.
//! - It provides multiple levels of assertion (SDL_assert, SDL_assert_release,
//!   SDL_assert_paranoid) instead of a single all-or-nothing option.
//! - It offers a variety of responses when an assertion fails (retry, trigger
//!   the debugger, abort the program, ignore the failure once, ignore it for
//!   the rest of the program's run).
//! - It tries to show the user a dialog by default, if possible, but the app
//!   can provide a callback to handle assertion failures however they like.
//! - It lets failed assertions be retried. Perhaps you had a network failure
//!   and just want to retry the test after plugging your network cable back
//!   in? You can.
//! - It lets the user ignore an assertion failure, if there's a harmless
//!   problem that one can continue past.
//! - It lets the user mark an assertion as ignored for the rest of the
//!   program's run; if there's a harmless problem that keeps popping up.
//! - It provides statistics and data on all failed assertions to the app.
//! - It allows the default assertion handler to be controlled with environment
//!   variables, in case an automated script needs to control it.
//!
//! To use it: do a debug build and just sprinkle around tests to check your
//! code!

use super::stdinc::*;

#[cfg(doc)]
emit! {
    /// The level of assertion aggressiveness.
    ///
    /// This value changes depending on compiler options and other preprocessor
    /// defines.
    ///
    /// It is currently one of the following values, but future SDL releases might
    /// add more:
    ///
    /// - 0: All SDL assertion macros are disabled.
    /// - 1: Release settings: SDL_assert disabled, SDL_assert_release enabled.
    /// - 2: Debug settings: SDL_assert and SDL_assert_release enabled.
    /// - 3: Paranoid settings: All SDL assertion macros enabled, including
    ///   SDL_assert_paranoid.
    ///
    /// \since This macro is available since SDL 3.0.0.
    pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 1;

}

#[cfg(not(doc))]
emit! {
    #[cfg(any(all(not(not(debug_assertions)), any(/* always disabled: __GNUC__ */)), debug_assertions, debug_assertions))]
    emit! {
    }

    #[cfg(not(any(all(not(not(debug_assertions)), any(/* always disabled: __GNUC__ */)), debug_assertions, debug_assertions)))]
    emit! {
    }

}

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {
    #[cfg(all(windows, target_env = "msvc"))]
    emit! {
    }

    #[cfg(not(all(windows, target_env = "msvc")))]
    emit! {
    }

}

#[cfg(all(windows, target_env = "msvc"))]
emit! {}

#[cfg(not(all(windows, target_env = "msvc")))]
emit! {}

#[cfg(all(not(doc), feature = "assert-level-disabled"))]
pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 0;
#[cfg(all(
    not(any(doc, feature = "assert-level-disabled")),
    feature = "assert-level-release"
))]
pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 1;
#[cfg(all(
    not(any(
        doc,
        feature = "assert-level-disabled",
        feature = "assert-level-release"
    )),
    feature = "assert-level-debug"
))]
pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 2;
#[cfg(all(
    not(any(
        doc,
        feature = "assert-level-disabled",
        feature = "assert-level-release",
        feature = "assert-level-debug"
    )),
    feature = "assert-level-paranoid"
))]
pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 3;

#[doc(hidden)]
#[macro_export]
macro_rules! SDL_disabled_assert {
    ($condition:expr) => {};
}
#[doc(inline)]
pub use SDL_disabled_assert;

/// Possible outcomes from a triggered assertion.
///
/// When an enabled assertion triggers, it may call the assertion handler
/// (possibly one provided by the app via SDL_SetAssertionHandler), which will
/// return one of these values, possibly after asking the user.
///
/// Then SDL will respond based on this outcome (loop around to retry the
/// condition, try to break in a debugger, kill the program, or ignore the
/// problem).
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_ASSERTION_RETRY`], [`SDL_ASSERTION_BREAK`], [`SDL_ASSERTION_ABORT`], [`SDL_ASSERTION_IGNORE`], [`SDL_ASSERTION_ALWAYS_IGNORE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AssertState(pub ::core::ffi::c_int);
impl SDL_AssertState {
    /// Retry the assert immediately.
    pub const RETRY: Self = Self(0);
    /// Make the debugger trigger a breakpoint.
    pub const BREAK: Self = Self(1);
    /// Terminate the program.
    pub const ABORT: Self = Self(2);
    /// Ignore the assert.
    pub const IGNORE: Self = Self(3);
    /// Ignore the assert from now on.
    pub const ALWAYS_IGNORE: Self = Self(4);
}
/// Retry the assert immediately.
pub const SDL_ASSERTION_RETRY: SDL_AssertState = SDL_AssertState::RETRY;
/// Make the debugger trigger a breakpoint.
pub const SDL_ASSERTION_BREAK: SDL_AssertState = SDL_AssertState::BREAK;
/// Terminate the program.
pub const SDL_ASSERTION_ABORT: SDL_AssertState = SDL_AssertState::ABORT;
/// Ignore the assert.
pub const SDL_ASSERTION_IGNORE: SDL_AssertState = SDL_AssertState::IGNORE;
/// Ignore the assert from now on.
pub const SDL_ASSERTION_ALWAYS_IGNORE: SDL_AssertState = SDL_AssertState::ALWAYS_IGNORE;

/// Information about an assertion failure.
///
/// This structure is filled in with information about a triggered assertion,
/// used by the assertion handler, then added to the assertion report. This is
/// returned as a linked list from SDL_GetAssertionReport().
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AssertData {
    /// true if app should always continue when assertion is triggered.
    pub always_ignore: ::core::primitive::bool,
    /// Number of times this assertion has been triggered.
    pub trigger_count: ::core::ffi::c_uint,
    /// A string of this assert's test code.
    pub condition: *const ::core::ffi::c_char,
    /// The source file where this assert lives.
    pub filename: *const ::core::ffi::c_char,
    /// The line in `filename` where this assert lives.
    pub linenum: ::core::ffi::c_int,
    /// The name of the function where this assert lives.
    pub function: *const ::core::ffi::c_char,
    /// next item in the linked list.
    pub next: *const SDL_AssertData,
}

extern "C" {
    /// Never call this directly.
    ///
    /// Use the SDL_assert* macros instead.
    ///
    /// \param data assert data structure.
    /// \param func function name.
    /// \param file file name.
    /// \param line line number.
    /// \returns assert state.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReportAssertion(
        data: *mut SDL_AssertData,
        func: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    ) -> SDL_AssertState;
}

#[inline(never)]
pub const unsafe fn SDL_AssertBreakpoint() {}

#[doc(hidden)]
#[macro_export]
macro_rules! SDL_enabled_assert {
    ($condition:expr) => {{
        while !$condition {
            // Yes, this is wildly unsafe, but it's fine! :thisisfine:
            // - SDL uses a mutex to protect access to SDL_AssertData
            // - The static mut can only be accessed through the pointer that's passed to SDL
            let assert_data = {
                #[repr(transparent)]
                struct SyncAssertData($crate::assert::SDL_AssertData);
                unsafe impl ::core::marker::Sync for SyncAssertData {}
                $crate::__const_c_str!(CONDITION = ::core::stringify!($condition));
                static mut SDL_ASSERT_DATA: SyncAssertData =
                    SyncAssertData($crate::assert::SDL_AssertData {
                        always_ignore: false,
                        trigger_count: 0,
                        condition: CONDITION.as_ptr(),
                        filename: ::core::ptr::null(),
                        linenum: 0,
                        function: ::core::ptr::null(),
                        next: ::core::ptr::null(),
                    });
                unsafe { ::core::ptr::addr_of_mut!(SDL_ASSERT_DATA.0) }
            };
            const LOCATION: &::core::panic::Location = ::core::panic::Location::caller();
            $crate::__const_c_str!(FILENAME = LOCATION.file());
            match unsafe {
                $crate::assert::SDL_ReportAssertion(
                    assert_data,
                    b"???\0".as_ptr().cast::<::core::ffi::c_char>(),
                    FILENAME.as_ptr(),
                    LOCATION.line() as ::core::ffi::c_int,
                )
            } {
                $crate::assert::SDL_ASSERTION_RETRY => continue,
                $crate::assert::SDL_ASSERTION_BREAK => unsafe {
                    $crate::assert::SDL_AssertBreakpoint()
                },
                _ => (),
            }
            break;
        }
    }};
}
#[doc(inline)]
pub use SDL_enabled_assert;

#[cfg(doc)]
emit! {
    /// An assertion test that is normally performed only in debug builds.
    ///
    /// This macro is enabled when the SDL_ASSERT_LEVEL is >= 2, otherwise it is
    /// disabled. This is meant to only do these tests in debug builds, so they can
    /// tend to be more expensive, and they are meant to bring everything to a halt
    /// when they fail, with the programmer there to assess the problem.
    ///
    /// In short: you can sprinkle these around liberally and assume they will
    /// evaporate out of the build when building for end-users.
    ///
    /// When assertions are disabled, this wraps `condition` in a `sizeof`
    /// operator, which means any function calls and side effects will not run, but
    /// the compiler will not complain about any otherwise-unused variables that
    /// are only referenced in the assertion.
    ///
    /// One can set the environment variable "SDL_ASSERT" to one of several strings
    /// ("abort", "break", "retry", "ignore", "always_ignore") to force a default
    /// behavior, which may be desirable for automation purposes. If your platform
    /// requires GUI interfaces to happen on the main thread but you're debugging
    /// an assertion in a background thread, it might be desirable to set this to
    /// "break" so that your debugger takes control as soon as assert is triggered,
    /// instead of risking a bad UI interaction (deadlock, etc) in the application.
    ///
    /// \param condition boolean value to test.
    ///
    /// \since This macro is available since SDL 3.0.0.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! SDL_assert {
        ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
    }
    #[doc(inline)]
    pub use SDL_assert;

    /// An assertion test that is performed even in release builds.
    ///
    /// This macro is enabled when the SDL_ASSERT_LEVEL is >= 1, otherwise it is
    /// disabled. This is meant to be for tests that are cheap to make and
    /// extremely unlikely to fail; generally it is frowned upon to have an
    /// assertion failure in a release build, so these assertions generally need to
    /// be of more than life-and-death importance if there's a chance they might
    /// trigger. You should almost always consider handling these cases more
    /// gracefully than an assert allows.
    ///
    /// When assertions are disabled, this wraps `condition` in a `sizeof`
    /// operator, which means any function calls and side effects will not run, but
    /// the compiler will not complain about any otherwise-unused variables that
    /// are only referenced in the assertion.
    ///
    /// One can set the environment variable "SDL_ASSERT" to one of several strings
    /// ("abort", "break", "retry", "ignore", "always_ignore") to force a default
    /// behavior, which may be desirable for automation purposes. If your platform
    /// requires GUI interfaces to happen on the main thread but you're debugging
    /// an assertion in a background thread, it might be desirable to set this to
    /// "break" so that your debugger takes control as soon as assert is triggered,
    /// instead of risking a bad UI interaction (deadlock, etc) in the application.
    /// *
    ///
    /// \param condition boolean value to test.
    ///
    /// \since This macro is available since SDL 3.0.0.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! SDL_assert_release {
        ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
    }
    #[doc(inline)]
    pub use SDL_assert_release;

    /// An assertion test that is performed only when built with paranoid settings.
    ///
    /// This macro is enabled when the SDL_ASSERT_LEVEL is >= 3, otherwise it is
    /// disabled. This is a higher level than both release and debug, so these
    /// tests are meant to be expensive and only run when specifically looking for
    /// extremely unexpected failure cases in a special build.
    ///
    /// When assertions are disabled, this wraps `condition` in a `sizeof`
    /// operator, which means any function calls and side effects will not run, but
    /// the compiler will not complain about any otherwise-unused variables that
    /// are only referenced in the assertion.
    ///
    /// One can set the environment variable "SDL_ASSERT" to one of several strings
    /// ("abort", "break", "retry", "ignore", "always_ignore") to force a default
    /// behavior, which may be desirable for automation purposes. If your platform
    /// requires GUI interfaces to happen on the main thread but you're debugging
    /// an assertion in a background thread, it might be desirable to set this to
    /// "break" so that your debugger takes control as soon as assert is triggered,
    /// instead of risking a bad UI interaction (deadlock, etc) in the application.
    ///
    /// \param condition boolean value to test.
    ///
    /// \since This macro is available since SDL 3.0.0.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! SDL_assert_paranoid {
        ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
    }
    #[doc(inline)]
    pub use SDL_assert_paranoid;
}

#[cfg(not(doc))]
emit! {
    #[cfg(feature = "assert-level-disabled")]
    emit! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! SDL_assert {
            ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
        }
        #[doc(inline)]
        pub use SDL_assert;

        #[doc(hidden)]
        #[macro_export]
        macro_rules! SDL_assert_release {
            ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
        }
        #[doc(inline)]
        pub use SDL_assert_release;

        #[doc(hidden)]
        #[macro_export]
        macro_rules! SDL_assert_paranoid {
            ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
        }
        #[doc(inline)]
        pub use SDL_assert_paranoid;
    }

    #[cfg(not(feature = "assert-level-disabled"))]
    emit! {
        #[cfg(feature = "assert-level-release")]
        emit! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! SDL_assert {
                ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
            }
            #[doc(inline)]
            pub use SDL_assert;

            #[doc(hidden)]
            #[macro_export]
            macro_rules! SDL_assert_release {
                ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
            }
            #[doc(inline)]
            pub use SDL_assert_release;

            #[doc(hidden)]
            #[macro_export]
            macro_rules! SDL_assert_paranoid {
                ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
            }
            #[doc(inline)]
            pub use SDL_assert_paranoid;
        }

        #[cfg(not(feature = "assert-level-release"))]
        emit! {
            #[cfg(feature = "assert-level-debug")]
            emit! {
                #[doc(hidden)]
                #[macro_export]
                macro_rules! SDL_assert {
                    ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
                }
                #[doc(inline)]
                pub use SDL_assert;

                #[doc(hidden)]
                #[macro_export]
                macro_rules! SDL_assert_release {
                    ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
                }
                #[doc(inline)]
                pub use SDL_assert_release;

                #[doc(hidden)]
                #[macro_export]
                macro_rules! SDL_assert_paranoid {
                    ($condition:expr) => { $crate::assert::SDL_disabled_assert!($condition) };
                }
                #[doc(inline)]
                pub use SDL_assert_paranoid;
            }

            #[cfg(not(feature = "assert-level-debug"))]
            emit! {
                #[cfg(feature = "assert-level-paranoid")]
                emit! {
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! SDL_assert {
                        ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
                    }
                    #[doc(inline)]
                    pub use SDL_assert;

                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! SDL_assert_release {
                        ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
                    }
                    #[doc(inline)]
                    pub use SDL_assert_release;

                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! SDL_assert_paranoid {
                        ($condition:expr) => { $crate::assert::SDL_enabled_assert!($condition) };
                    }
                    #[doc(inline)]
                    pub use SDL_assert_paranoid;
                }

                #[cfg(not(feature = "assert-level-paranoid"))]
                emit! {
                    ::core::compile_error!("Unknown assertion level.");
                }

            }

        }

    }

}

/// An assertion test that always performed.
///
/// This macro is always enabled no matter what SDL_ASSERT_LEVEL is set to. You
/// almost never want to use this, as it could trigger on an end-user's system,
/// crashing your program.
///
/// One can set the environment variable "SDL_ASSERT" to one of several strings
/// ("abort", "break", "retry", "ignore", "always_ignore") to force a default
/// behavior, which may be desirable for automation purposes. If your platform
/// requires GUI interfaces to happen on the main thread but you're debugging
/// an assertion in a background thread, it might be desirable to set this to
/// "break" so that your debugger takes control as soon as assert is triggered,
/// instead of risking a bad UI interaction (deadlock, etc) in the application.
///
/// \param condition boolean value to test.
///
/// \since This macro is available since SDL 3.0.0.
#[doc(hidden)]
#[macro_export]
macro_rules! SDL_assert_always {
    ($condition:expr) => {
        $crate::assert::SDL_enabled_assert!($condition)
    };
}
#[doc(inline)]
pub use SDL_assert_always;

/// A callback that fires when an SDL assertion fails.
///
/// \param data a pointer to the SDL_AssertData structure corresponding to the
///             current assertion.
/// \param userdata what was passed as `userdata` to SDL_SetAssertionHandler().
/// \returns an SDL_AssertState value indicating how to handle the failure.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_AssertionHandler = ::core::option::Option<
    extern "C" fn(
        data: *const SDL_AssertData,
        userdata: *mut ::core::ffi::c_void,
    ) -> SDL_AssertState,
>;

extern "C" {
    /// Set an application-defined assertion handler.
    ///
    /// This function allows an application to show its own assertion UI and/or
    /// force the response to an assertion failure. If the application doesn't
    /// provide this, SDL will try to do the right thing, popping up a
    /// system-specific GUI dialog, and probably minimizing any fullscreen windows.
    ///
    /// This callback may fire from any thread, but it runs wrapped in a mutex, so
    /// it will only fire from one thread at a time.
    ///
    /// This callback is NOT reset to SDL's internal handler upon SDL_Quit()!
    ///
    /// \param handler the SDL_AssertionHandler function to call when an assertion
    ///                fails or NULL for the default handler.
    /// \param userdata a pointer that is passed to `handler`.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionHandler
    pub fn SDL_SetAssertionHandler(
        handler: SDL_AssertionHandler,
        userdata: *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Get the default assertion handler.
    ///
    /// This returns the function pointer that is called by default when an
    /// assertion is triggered. This is an internal function provided by SDL, that
    /// is used for assertions when SDL_SetAssertionHandler() hasn't been used to
    /// provide a different function.
    ///
    /// \returns the default SDL_AssertionHandler that is called when an assert
    ///          triggers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionHandler
    pub fn SDL_GetDefaultAssertionHandler() -> SDL_AssertionHandler;
}

extern "C" {
    /// Get the current assertion handler.
    ///
    /// This returns the function pointer that is called when an assertion is
    /// triggered. This is either the value last passed to
    /// SDL_SetAssertionHandler(), or if no application-specified function is set,
    /// is equivalent to calling SDL_GetDefaultAssertionHandler().
    ///
    /// The parameter `puserdata` is a pointer to a void*, which will store the
    /// "userdata" pointer that was passed to SDL_SetAssertionHandler(). This value
    /// will always be NULL for the default handler. If you don't care about this
    /// data, it is safe to pass a NULL pointer to this function to ignore it.
    ///
    /// \param puserdata pointer which is filled with the "userdata" pointer that
    ///                  was passed to SDL_SetAssertionHandler().
    /// \returns the SDL_AssertionHandler that is called when an assert triggers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetAssertionHandler
    pub fn SDL_GetAssertionHandler(
        puserdata: *mut *mut ::core::ffi::c_void,
    ) -> SDL_AssertionHandler;
}

extern "C" {
    /// Get a list of all assertion failures.
    ///
    /// This function gets all assertions triggered since the last call to
    /// SDL_ResetAssertionReport(), or the start of the program.
    ///
    /// The proper way to examine this data looks something like this:
    ///
    /// ```c
    /// const SDL_AssertData *item = SDL_GetAssertionReport();
    /// while (item) {
    ///    printf("'%s', %s (%s:%d), triggered %u times, always ignore: %s.\\n",
    ///           item->condition, item->function, item->filename,
    ///           item->linenum, item->trigger_count,
    ///           item->always_ignore ? "yes" : "no");
    ///    item = item->next;
    /// }
    /// ```
    ///
    /// \returns a list of all failed assertions or NULL if the list is empty. This
    ///          memory should not be modified or freed by the application.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ResetAssertionReport
    pub fn SDL_GetAssertionReport() -> *const SDL_AssertData;
}

extern "C" {
    /// Clear the list of all assertion failures.
    ///
    /// This function will clear the list of all assertions triggered up to that
    /// point. Immediately following this call, SDL_GetAssertionReport will return
    /// no items. In addition, any previously-triggered assertions will be reset to
    /// a trigger_count of zero, and their always_ignore state will be false.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionReport
    pub fn SDL_ResetAssertionReport();
}
