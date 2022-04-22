//! global variable definition

/********************
 * Definition of Boundary Values of Integer Types in Different Languages
 ********************/

pub const MIN_UINT: u128 = 0;

/// Rust: unsigned integer values
pub const RUST_MAX_U8: u128 = 255;
pub const RUST_MAX_U16: u128 = 65535;
pub const RUST_MAX_U32: u128 = 4294967295;
pub const RUST_MAX_U64: u128 = 18446744073709551615;
pub const RUST_MAX_U128: u128 = 340282366920938463463374607431768211455;

/// Rust: signed integer values
pub const RUST_MAX_I8: i128 = 127;
pub const RUST_MIN_I8: i128 = -128;
pub const RUST_MAX_I16: i128 = 32767;
pub const RUST_MIN_I16: i128 = -32768;
pub const RUST_MAX_I32: i128 = 2147483647;
pub const RUST_MIN_I32: i128 = -2147483648;
pub const RUST_MAX_I64: i128 = 9223372036854775807;
pub const RUST_MIN_I64: i128 = -9223372036854775808;
pub const RUST_MAX_I128: i128 = 170141183460469231731687303715884105727;
pub const RUST_MIN_I128: i128 = -170141183460469231731687303715884105728;

// C/C++: unsigned integer values
pub const C_MAX_UINT8: u128 = 255;
pub const C_MAX_USHORT_UINT16: u128 = 65535;
pub const C_MAX_UINT_ULONG_UINT32: u128 = 4294967295;
pub const C_MAX_ULONGLONG_UINT64: u128 = 18446744073709551615;

// C/C++: signed integer values
pub const C_MAX_INT8: i128 = 127;
pub const C_MIN_INT8: i128 = -128;
pub const C_MAX_SHORT_INT16: i128 = 32767;
pub const C_MIN_SHORT_INT16: i128 = -32768;
pub const C_MAX_INT_LONG_INT32: i128 = 2147483647;
pub const C_MIN_INT_LONG_INT32: i128 = -2147483648;
pub const C_MAX_LONGLONG_INT64: i128 = 9223372036854775807;
pub const C_MIN_LONGLONG_INT64: i128 = -9223372036854775808;

// Golang: unsigned integer values
pub const GO_MAX_UINT8: u128 = 255;
pub const GO_MAX_UINT16: u128 = 65535;
pub const GO_MAX_UINT32: u128 = 4294967295;
pub const GO_MAX_UINT64: u128 = 18446744073709551615;

// Golang: signed integer values
pub const GO_MAX_INT8: i128 = 127;
pub const GO_MIN_INT8: i128 = -128;
pub const GO_MAX_INT16: i128 = 32767;
pub const GO_MIN_INT16: i128 = -32768;
pub const GO_MAX_INT32: i128 = 2147483647;
pub const GO_MIN_INT32: i128 = -2147483648;
pub const GO_MAX_INT64: i128 = 9223372036854775807;
pub const GO_MIN_INT64: i128 = -9223372036854775808;
