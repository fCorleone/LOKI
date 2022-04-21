//! global variable definition

/*
 * Definition of Boundary Values of Integer Types in Different Languages
 */

/// Rust: unsigned integer values
pub const RUST_MAX_U8: u8 = 255;
pub const RUST_MIN_U8: u8 = 0;
pub const RUST_MAX_U16: u16 = 65535;
pub const RUST_MIN_U16: u16 = 0;
pub const RUST_MAX_U32: u32 = 4294967295;
pub const RUST_MIN_U32: u32 = 0;
pub const RUST_MAX_U64: u64 = 18446744073709551615;
pub const RUST_MIN_U64: u64 = 0;
pub const RUST_MAX_U128: u128 = 340282366920938463463374607431768211455;
pub const RUST_MIN_U128: u128 = 0;

/// Rust: signed integer values
pub const RUST_MAX_I8: i8 = 127;
pub const RUST_MIN_I8: i8 = -128;
pub const RUST_MAX_I16: i16 = 32767;
pub const RUST_MIN_I16: i16 = -32768;
pub const RUST_MAX_I32: i32 = 2147483647;
pub const RUST_MIN_I32: i32 = -2147483648;
pub const RUST_MAX_I64: i64 = 9223372036854775807;
pub const RUST_MIN_I64: i64 = -9223372036854775808;
pub const RUST_MAX_I128: i128 = 170141183460469231731687303715884105727;
pub const RUST_MIN_I128: i128 = -170141183460469231731687303715884105728;

// C/C++: unsigned integer values
pub const C_MAX_USHORT: u128 = 65535;
pub const C_MIN_USHORT: u128 = 0;
pub const C_MAX_UINT: u128 = 4294967295;
pub const C_MIN_UINT: u128 = 0;
pub const C_MAX_ULONG: u128 = 4294967295;
pub const C_MIN_ULONG: u128 = 0;
pub const C_MAX_ULONGLONG: u128 = 18446744073709551615;
pub const C_MIN_ULONGLONG: u128 = 0;
pub const C_MAX_UI64: u128 = 18446744073709551615;
pub const C_MIN_UI64: u128 = 0;

// C/C++: signed integer values
pub const C_MAX_SHORT: i128 = 32767;
pub const C_MIN_SHORT: i128 = -32768;
pub const C_MAX_INT: i128 = 2147483647;
pub const C_MIN_INT: i128 = -2147483648;
pub const C_MAX_LONG: i128 = 2147483647;
pub const C_MIN_LONG: i128 = -2147483648;
pub const C_MAX_LONGLONG: i128 = 9223372036854775807;
pub const C_MIN_LONGLONG: i128 = -9223372036854775808;
pub const C_MAX_I64: i128 = 9223372036854775807;
pub const C_MIN_I64: i128 = -9223372036854775808;

// Golang: unsigned integer values
pub const GO_MAX_UINT8: u128 = 255;
pub const GO_MIN_UINT8: u128 = 0;
pub const GO_MAX_UINT16: u128 = 65535;
pub const GO_MIN_UINT16: u128 = 0;
pub const GO_MAX_UINT32: u128 = 4294967295;
pub const GO_MIN_UINT32: u128 = 0;
pub const GO_MAX_UINT64: u128 = 18446744073709551615;
pub const GO_MIN_UINT64: u128 = 0;

// Golang: signed integer values
pub const GO_MAX_INT8: i128 = 127;
pub const GO_MIN_INT8: i128 = -128;
pub const GO_MAX_INT16: i128 = 32767;
pub const GO_MIN_INT16: i128 = -32768;
pub const GO_MAX_INT32: i128 = 2147483647;
pub const GO_MIN_INT32: i128 = -2147483648;
pub const GO_MAX_INT64: i128 = 9223372036854775807;
pub const GO_MIN_INT64: i128 = -9223372036854775808;
