#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategorySensor
//!
//! SDL sensor management.

use super::stdinc::*;

use super::error::*;

use super::properties::*;

/// This is a unique ID for a sensor for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// The value 0 is an invalid ID.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_SensorID = Uint32;

pub const SDL_STANDARD_GRAVITY: ::core::primitive::f32 = 9.80665;

/// The different sensors defined by SDL.
///
/// Additional sensors may be available, using platform dependent semantics.
///
/// Here are the additional Android sensors:
///
/// https://developer.android.com/reference/android/hardware/SensorEvent.html#values
///
/// Accelerometer sensor notes:
///
/// The accelerometer returns the current acceleration in SI meters per second
/// squared. This measurement includes the force of gravity, so a device at
/// rest will have an value of SDL_STANDARD_GRAVITY away from the center of the
/// earth, which is a positive Y value.
///
/// - `values[0]`: Acceleration on the x axis
/// - `values[1]`: Acceleration on the y axis
/// - `values[2]`: Acceleration on the z axis
///
/// For phones and tablets held in natural orientation and game controllers
/// held in front of you, the axes are defined as follows:
///
/// - -X ... +X : left ... right
/// - -Y ... +Y : bottom ... top
/// - -Z ... +Z : farther ... closer
///
/// The accelerometer axis data is not changed when the device is rotated.
///
/// Gyroscope sensor notes:
///
/// The gyroscope returns the current rate of rotation in radians per second.
/// The rotation is positive in the counter-clockwise direction. That is, an
/// observer looking from a positive location on one of the axes would see
/// positive rotation on that axis when it appeared to be rotating
/// counter-clockwise.
///
/// - `values[0]`: Angular speed around the x axis (pitch)
/// - `values[1]`: Angular speed around the y axis (yaw)
/// - `values[2]`: Angular speed around the z axis (roll)
///
/// For phones and tablets held in natural orientation and game controllers
/// held in front of you, the axes are defined as follows:
///
/// - -X ... +X : left ... right
/// - -Y ... +Y : bottom ... top
/// - -Z ... +Z : farther ... closer
///
/// The gyroscope axis data is not changed when the device is rotated.
///
/// \since This enum is available since SDL 3.0.0.
///
/// \sa SDL_GetCurrentDisplayOrientation
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_SENSOR_INVALID`], [`SDL_SENSOR_UNKNOWN`], [`SDL_SENSOR_ACCEL`], [`SDL_SENSOR_GYRO`], [`SDL_SENSOR_ACCEL_L`], [`SDL_SENSOR_GYRO_L`], [`SDL_SENSOR_ACCEL_R`], [`SDL_SENSOR_GYRO_R`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_SensorType(pub ::core::ffi::c_int);
impl SDL_SensorType {
    /// Returned for an invalid sensor
    pub const INVALID: Self = Self(-1_i32);
    /// Unknown sensor type
    pub const UNKNOWN: Self = Self(0_i32);
    /// Accelerometer
    pub const ACCEL: Self = Self(1_i32);
    /// Gyroscope
    pub const GYRO: Self = Self(2_i32);
    /// Accelerometer for left Joy-Con controller and Wii nunchuk
    pub const ACCEL_L: Self = Self(3_i32);
    /// Gyroscope for left Joy-Con controller
    pub const GYRO_L: Self = Self(4_i32);
    /// Accelerometer for right Joy-Con controller
    pub const ACCEL_R: Self = Self(5_i32);
    /// Gyroscope for right Joy-Con controller
    pub const GYRO_R: Self = Self(6_i32);
}
/// Returned for an invalid sensor
pub const SDL_SENSOR_INVALID: SDL_SensorType = SDL_SensorType::INVALID;
/// Unknown sensor type
pub const SDL_SENSOR_UNKNOWN: SDL_SensorType = SDL_SensorType::UNKNOWN;
/// Accelerometer
pub const SDL_SENSOR_ACCEL: SDL_SensorType = SDL_SensorType::ACCEL;
/// Gyroscope
pub const SDL_SENSOR_GYRO: SDL_SensorType = SDL_SensorType::GYRO;
/// Accelerometer for left Joy-Con controller and Wii nunchuk
pub const SDL_SENSOR_ACCEL_L: SDL_SensorType = SDL_SensorType::ACCEL_L;
/// Gyroscope for left Joy-Con controller
pub const SDL_SENSOR_GYRO_L: SDL_SensorType = SDL_SensorType::GYRO_L;
/// Accelerometer for right Joy-Con controller
pub const SDL_SENSOR_ACCEL_R: SDL_SensorType = SDL_SensorType::ACCEL_R;
/// Gyroscope for right Joy-Con controller
pub const SDL_SENSOR_GYRO_R: SDL_SensorType = SDL_SensorType::GYRO_R;

extern_sdlcall! {{
    /// Get a list of currently connected sensors.
    ///
    /// \param count a pointer filled in with the number of sensors returned, may
    ///              be NULL.
    /// \returns a 0 terminated array of sensor instance IDs or NULL on failure;
    ///          call SDL_GetError() for more information. This should be freed
    ///          with SDL_free() when it is no longer needed.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensors(count: *mut ::core::ffi::c_int) -> *mut SDL_SensorID;
}}

extern_sdlcall! {{
    /// Get the implementation dependent name of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// \param instance_id the sensor instance ID.
    /// \returns the sensor name, or NULL if `instance_id` is not valid.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorNameForID(instance_id: SDL_SensorID) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the type of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// \param instance_id the sensor instance ID.
    /// \returns the SDL_SensorType, or `SDL_SENSOR_INVALID` if `instance_id` is
    ///          not valid.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorTypeForID(instance_id: SDL_SensorID) -> SDL_SensorType;
}}

extern_sdlcall! {{
    /// Get the platform dependent type of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// \param instance_id the sensor instance ID.
    /// \returns the sensor platform dependent type, or -1 if `instance_id` is not
    ///          valid.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorNonPortableTypeForID(instance_id: SDL_SensorID) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Open a sensor for use.
    ///
    /// \param instance_id the sensor instance ID.
    /// \returns an SDL_Sensor object or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_OpenSensor(instance_id: SDL_SensorID) -> *mut SDL_Sensor;
}}

extern_sdlcall! {{
    /// Return the SDL_Sensor associated with an instance ID.
    ///
    /// \param instance_id the sensor instance ID.
    /// \returns an SDL_Sensor object or NULL on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorFromID(instance_id: SDL_SensorID) -> *mut SDL_Sensor;
}}

extern_sdlcall! {{
    /// Get the properties associated with a sensor.
    ///
    /// \param sensor the SDL_Sensor object.
    /// \returns a valid property ID on success or 0 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorProperties(sensor: *mut SDL_Sensor) -> SDL_PropertiesID;
}}

extern_sdlcall! {{
    /// Get the implementation dependent name of a sensor.
    ///
    /// \param sensor the SDL_Sensor object.
    /// \returns the sensor name or NULL on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorName(sensor: *mut SDL_Sensor) -> *const ::core::ffi::c_char;
}}

extern_sdlcall! {{
    /// Get the type of a sensor.
    ///
    /// \param sensor the SDL_Sensor object to inspect.
    /// \returns the SDL_SensorType type, or `SDL_SENSOR_INVALID` if `sensor` is
    ///          NULL.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorType(sensor: *mut SDL_Sensor) -> SDL_SensorType;
}}

extern_sdlcall! {{
    /// Get the platform dependent type of a sensor.
    ///
    /// \param sensor the SDL_Sensor object to inspect.
    /// \returns the sensor platform dependent type, or -1 if `sensor` is NULL.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorNonPortableType(sensor: *mut SDL_Sensor) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the instance ID of a sensor.
    ///
    /// \param sensor the SDL_Sensor object to inspect.
    /// \returns the sensor instance ID, or 0 on failure; call SDL_GetError() for
    ///          more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorID(sensor: *mut SDL_Sensor) -> SDL_SensorID;
}}

extern_sdlcall! {{
    /// Get the current state of an opened sensor.
    ///
    /// The number of values and interpretation of the data is sensor dependent.
    ///
    /// \param sensor the SDL_Sensor object to query.
    /// \param data a pointer filled with the current sensor state.
    /// \param num_values the number of values to write to data.
    /// \returns SDL_TRUE on success or SDL_FALSE on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_GetSensorData(sensor: *mut SDL_Sensor, data: *mut ::core::ffi::c_float, num_values: ::core::ffi::c_int) -> SDL_bool;
}}

extern_sdlcall! {{
    /// Close a sensor previously opened with SDL_OpenSensor().
    ///
    /// \param sensor the SDL_Sensor object to close.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_CloseSensor(sensor: *mut SDL_Sensor);
}}

extern_sdlcall! {{
    /// Update the current state of the open sensors.
    ///
    /// This is called automatically by the event loop if sensor events are
    /// enabled.
    ///
    /// This needs to be called from the thread that initialized the sensor
    /// subsystem.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_UpdateSensors();
}}

#[repr(C)]
#[non_exhaustive]
pub struct SDL_Sensor { _opaque: [::core::primitive::u8; 0] }
