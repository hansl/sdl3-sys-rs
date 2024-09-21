//! Process control support.
//!
//! These functions provide a cross-platform way to spawn and manage OS-level
//! processes.
//!
//! You can create a new subprocess with SDL_CreateProcess() and optionally
//! read and write to it using SDL_ReadProcess() or SDL_GetProcessInput() and
//! SDL_GetProcessOutput(). If more advanced functionality like chaining input
//! between processes is necessary, you can use
//! SDL_CreateProcessWithProperties().
//!
//! You can get the status of a created process with SDL_WaitProcess(), or
//! terminate the process with SDL_KillProcess().
//!
//! Don't forget to call SDL_DestroyProcess() to clean up, whether the process
//! process was killed, terminated on its own, or is still running!

use super::stdinc::*;

use super::error::*;

use super::iostream::*;

use super::properties::*;

extern "C" {
    /// Create a new process.
    ///
    /// The path to the executable is supplied in args[0]. args[1..N] are
    /// additional arguments passed on the command line of the new process, and the
    /// argument list should be terminated with a NULL, e.g.:
    ///
    /// ```c
    /// const char *args[] = { "myprogram", "argument", NULL };
    /// ```
    ///
    /// Setting pipe_stdio to true is equivalent to setting
    /// `SDL_PROP_PROCESS_CREATE_STDIN_NUMBER` and
    /// `SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER` to `SDL_PROCESS_STDIO_APP`, and
    /// will allow the use of SDL_ReadProcess() or SDL_GetProcessInput() and
    /// SDL_GetProcessOutput().
    ///
    /// See SDL_CreateProcessWithProperties() for more details.
    ///
    /// \param args the path and arguments for the new process.
    /// \param pipe_stdio true to create pipes to the process's standard input and
    ///                   from the process's standard output, false for the process
    ///                   to have no input and inherit the application's standard
    ///                   output.
    /// \returns the newly created and running process, or NULL if the process
    ///          couldn't be created.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_GetProcessProperties
    /// \sa SDL_ReadProcess
    /// \sa SDL_GetProcessInput
    /// \sa SDL_GetProcessOutput
    /// \sa SDL_KillProcess
    /// \sa SDL_WaitProcess
    /// \sa SDL_DestroyProcess
    pub fn SDL_CreateProcess(
        args: *const *const ::core::ffi::c_char,
        pipe_stdio: ::core::primitive::bool,
    ) -> *mut SDL_Process;
}

/// Description of where standard I/O should be directed when creating a
/// process.
///
/// If a standard I/O stream is set to SDL_PROCESS_STDIO_INHERIT, it will go to
/// the same place as the application's I/O stream. This is the default for
/// standard output and standard error.
///
/// If a standard I/O stream is set to SDL_PROCESS_STDIO_NULL, it is connected
/// to `NUL:` on Windows and `/dev/null` on POSIX systems. This is the default
/// for standard input.
///
/// If a standard I/O stream is set to SDL_PROCESS_STDIO_APP, it is connected
/// to a new SDL_IOStream that is available to the application. Standard input
/// will be available as `SDL_PROP_PROCESS_STDIN_POINTER` and allows
/// SDL_GetProcessInput(), standard output will be available as
/// `SDL_PROP_PROCESS_STDOUT_POINTER` and allows SDL_ReadProcess() and
/// SDL_GetProcessOutput(), and standard error will be available as
/// `SDL_PROP_PROCESS_STDERR_POINTER` in the properties for the created
/// process.
///
/// If a standard I/O stream is set to SDL_PROCESS_STDIO_REDIRECT, it is
/// connected to an existing SDL_IOStream provided by the application. Standard
/// input is provided using `SDL_PROP_PROCESS_CREATE_STDIN_POINTER`, standard
/// output is provided using `SDL_PROP_PROCESS_CREATE_STDOUT_POINTER`, and
/// standard error is provided using `SDL_PROP_PROCESS_CREATE_STDERR_POINTER`
/// in the creation properties. These existing streams should be closed by the
/// application once the new process is created.
///
/// In order to use an SDL_IOStream with SDL_PROCESS_STDIO_REDIRECT, it must
/// have `SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER` or
/// `SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER` set. This is true for streams
/// representing files and process I/O.
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_CreateProcessWithProperties
/// \sa SDL_GetProcessProperties
/// \sa SDL_ReadProcess
/// \sa SDL_GetProcessInput
/// \sa SDL_GetProcessOutput
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_PROCESS_STDIO_INHERITED`], [`SDL_PROCESS_STDIO_NULL`], [`SDL_PROCESS_STDIO_APP`], [`SDL_PROCESS_STDIO_REDIRECT`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_ProcessIO(pub ::core::ffi::c_int);
impl SDL_ProcessIO {
    /// The I/O stream is inherited from the application.
    pub const INHERITED: Self = Self(0);
    /// The I/O stream is ignored.
    pub const NULL: Self = Self(1);
    /// The I/O stream is connected to a new SDL_IOStream that the application can read or write
    pub const APP: Self = Self(2);
    /// The I/O stream is redirected to an existing SDL_IOStream.
    pub const REDIRECT: Self = Self(3);
}
/// The I/O stream is inherited from the application.
pub const SDL_PROCESS_STDIO_INHERITED: SDL_ProcessIO = SDL_ProcessIO::INHERITED;
/// The I/O stream is ignored.
pub const SDL_PROCESS_STDIO_NULL: SDL_ProcessIO = SDL_ProcessIO::NULL;
/// The I/O stream is connected to a new SDL_IOStream that the application can read or write
pub const SDL_PROCESS_STDIO_APP: SDL_ProcessIO = SDL_ProcessIO::APP;
/// The I/O stream is redirected to an existing SDL_IOStream.
pub const SDL_PROCESS_STDIO_REDIRECT: SDL_ProcessIO = SDL_ProcessIO::REDIRECT;

extern "C" {
    /// Create a new process with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - `SDL_PROP_PROCESS_CREATE_ARGS_POINTER`: an array of strings containing
    ///   the program to run, any arguments, and a NULL pointer, e.g. const char
    ///   *args[] = { "myprogram", "argument", NULL }. This is a required property.
    /// - `SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER`: an SDL_Environment
    ///   pointer. If this property is set, it will be the entire environment for
    ///   the process, otherwise the current environment is used.
    /// - `SDL_PROP_PROCESS_CREATE_STDIN_NUMBER`: an SDL_ProcessIO value describing
    ///   where standard input for the process comes from, defaults to
    ///   `SDL_PROCESS_STDIO_NULL`.
    /// - `SDL_PROP_PROCESS_CREATE_STDIN_POINTER`: an SDL_IOStream pointer used for
    ///   standard input when `SDL_PROP_PROCESS_CREATE_STDIN_NUMBER` is set to
    ///   `SDL_PROCESS_STDIO_REDIRECT`.
    /// - `SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER`: an SDL_ProcessIO value
    ///   describing where standard output for the process goes go, defaults to
    ///   `SDL_PROCESS_STDIO_INHERITED`.
    /// - `SDL_PROP_PROCESS_CREATE_STDOUT_POINTER`: an SDL_IOStream pointer used
    ///   for standard output when `SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER` is set
    ///   to `SDL_PROCESS_STDIO_REDIRECT`.
    /// - `SDL_PROP_PROCESS_CREATE_STDERR_NUMBER`: an SDL_ProcessIO value
    ///   describing where standard error for the process goes go, defaults to
    ///   `SDL_PROCESS_STDIO_INHERITED`.
    /// - `SDL_PROP_PROCESS_CREATE_STDERR_POINTER`: an SDL_IOStream pointer used
    ///   for standard error when `SDL_PROP_PROCESS_CREATE_STDERR_NUMBER` is set to
    ///   `SDL_PROCESS_STDIO_REDIRECT`.
    /// - `SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN`: true if the error
    ///   output of the process should be redirected into the standard output of
    ///   the process. This property has no effect if
    ///   `SDL_PROP_PROCESS_CREATE_STDERR_NUMBER` is set.
    /// - `SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN`: true if the process should
    ///   run in the background. In this case the default input and output is
    ///   `SDL_PROCESS_STDIO_NULL` and the exitcode of the process is not
    ///   available, and will always be 0.
    ///
    /// On POSIX platforms, wait() and waitpid(-1, ...) should not be called, and
    /// SIGCHLD should not be ignored or handled because those would prevent SDL
    /// from properly tracking the lifetime of the underlying process. You should
    /// use SDL_WaitProcess() instead.
    ///
    /// \param props the properties to use.
    /// \returns the newly created and running process, or NULL if the process
    ///          couldn't be created.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_GetProcessProperties
    /// \sa SDL_ReadProcess
    /// \sa SDL_GetProcessInput
    /// \sa SDL_GetProcessOutput
    /// \sa SDL_KillProcess
    /// \sa SDL_WaitProcess
    /// \sa SDL_DestroyProcess
    pub fn SDL_CreateProcessWithProperties(props: SDL_PropertiesID) -> *mut SDL_Process;
}

pub const SDL_PROP_PROCESS_CREATE_ARGS_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.args\0") };

pub const SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.environment\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDIN_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stdin_option\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDIN_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stdin_source\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stdout_option\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDOUT_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stdout_source\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDERR_NUMBER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stderr_option\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDERR_POINTER: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stderr_source\0")
};

pub const SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN: &::core::ffi::CStr = unsafe {
    ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.stderr_to_stdout\0")
};

pub const SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.create.background\0") };

extern "C" {
    /// Get the properties associated with a process.
    ///
    /// The following read-only properties are provided by SDL:
    ///
    /// - `SDL_PROP_PROCESS_PID_NUMBER`: the process ID of the process.
    /// - `SDL_PROP_PROCESS_STDIN_POINTER`: an SDL_IOStream that can be used to write input to the process, if it was created with `SDL_PROP_PROCESS_CREATE_STDIN_NUMBER` set to `SDL_PROCESS_STDIO_APP`.
    /// - `SDL_PROP_PROCESS_STDOUT_POINTER`: a non-blocking SDL_IOStream that can be used to read output from the process, if it was created with `SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER` set to `SDL_PROCESS_STDIO_APP`.
    /// - `SDL_PROP_PROCESS_STDERR_POINTER`: a non-blocking SDL_IOStream that can be used to read error output from the process, if it was created with `SDL_PROP_PROCESS_CREATE_STDERR_NUMBER` set to `SDL_PROCESS_STDIO_APP`.
    /// - `SDL_PROP_PROCESS_BACKGROUND_BOOLEAN`: true if the process is running in the background.
    ///
    /// \param process the process to query.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetProcessProperties(process: *mut SDL_Process) -> SDL_PropertiesID;
}

pub const SDL_PROP_PROCESS_PID_NUMBER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.pid\0") };

pub const SDL_PROP_PROCESS_STDIN_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.stdin\0") };

pub const SDL_PROP_PROCESS_STDOUT_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.stdout\0") };

pub const SDL_PROP_PROCESS_STDERR_POINTER: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.stderr\0") };

pub const SDL_PROP_PROCESS_BACKGROUND_BOOLEAN: &::core::ffi::CStr =
    unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(b"SDL.process.background\0") };

extern "C" {
    /// Read all the output from a process.
    ///
    /// If a process was created with I/O enabled, you can use this function to
    /// read the output. This function blocks until the process is complete,
    /// capturing all output, and providing the process exit code.
    ///
    /// The data is allocated with a zero byte at the end (null terminated) for
    /// convenience. This extra byte is not included in the value reported via
    /// `datasize`.
    ///
    /// The data should be freed with SDL_free().
    ///
    /// \param process The process to read.
    /// \param datasize a pointer filled in with the number of bytes read, may be
    ///                 NULL.
    /// \param exitcode a pointer filled in with the process exit code if the
    ///                 process has exited, may be NULL.
    /// \returns the data or NULL on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety This function is not thread safe.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_DestroyProcess
    pub fn SDL_ReadProcess(
        process: *mut SDL_Process,
        datasize: *mut ::core::primitive::usize,
        exitcode: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Get the SDL_IOStream associated with process standard input.
    ///
    /// The process must have been created with SDL_CreateProcess() and pipe_stdio
    /// set to true, or with SDL_CreateProcessWithProperties() and
    /// `SDL_PROP_PROCESS_CREATE_STDIN_NUMBER` set to `SDL_PROCESS_STDIO_APP`.
    ///
    /// Writing to this stream can return less data than expected if the process
    /// hasn't read its input. It may be blocked waiting for its output to be read,
    /// so if you may need to call SDL_GetOutputStream() and read the output in
    /// parallel with writing input.
    ///
    /// \param process The process to get the input stream for.
    /// \returns the input stream or NULL on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_GetProcessOutput
    pub fn SDL_GetProcessInput(process: *mut SDL_Process) -> *mut SDL_IOStream;
}

extern "C" {
    /// Get the SDL_IOStream associated with process standard output.
    ///
    /// The process must have been created with SDL_CreateProcess() and pipe_stdio
    /// set to true, or with SDL_CreateProcessWithProperties() and
    /// `SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER` set to `SDL_PROCESS_STDIO_APP`.
    ///
    /// Reading from this stream can return 0 with SDL_GetIOStatus() returning
    /// SDL_IO_STATUS_NOT_READY if no output is available yet.
    ///
    /// \param process The process to get the output stream for.
    /// \returns the output stream or NULL on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety It is safe to call this function from any thread.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_GetProcessInput
    pub fn SDL_GetProcessOutput(process: *mut SDL_Process) -> *mut SDL_IOStream;
}

extern "C" {
    /// Stop a process.
    ///
    /// \param process The process to stop.
    /// \param force true to terminate the process immediately, false to try to
    ///              stop the process gracefully. In general you should try to stop
    ///              the process gracefully first as terminating a process may
    ///              leave it with half-written data or in some other unstable
    ///              state.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \threadsafety This function is not thread safe.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_WaitProcess
    /// \sa SDL_DestroyProcess
    pub fn SDL_KillProcess(
        process: *mut SDL_Process,
        force: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Wait for a process to finish.
    ///
    /// This can be called multiple times to get the status of a process.
    ///
    /// The exit code will be the exit code of the process if it terminates
    /// normally, a negative signal if it terminated due to a signal, or -255
    /// otherwise. It will not be changed if the process is still running.
    ///
    /// \param process The process to wait for.
    /// \param block If true, block until the process finishes; otherwise, report
    ///              on the process' status.
    /// \param exitcode a pointer filled in with the process exit code if the
    ///                 process has exited, may be NULL.
    /// \returns true if the process exited, false otherwise.
    ///
    /// \threadsafety This function is not thread safe.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_KillProcess
    /// \sa SDL_DestroyProcess
    pub fn SDL_WaitProcess(
        process: *mut SDL_Process,
        block: ::core::primitive::bool,
        exitcode: *mut ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a previously created process object.
    ///
    /// Note that this does not stop the process, just destroys the SDL object used
    /// to track it. If you want to stop the process you should use
    /// SDL_KillProcess().
    ///
    /// \param process The process object to destroy.
    ///
    /// \threadsafety This function is not thread safe.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_CreateProcess
    /// \sa SDL_CreateProcessWithProperties
    /// \sa SDL_KillProcess
    pub fn SDL_DestroyProcess(process: *mut SDL_Process);
}

#[repr(C)]
#[non_exhaustive]
pub struct SDL_Process {
    _opaque: [::core::primitive::u8; 0],
}