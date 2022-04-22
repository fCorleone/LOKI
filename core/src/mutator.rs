//! Mutator used by LOKI to mutate the existing seeds or generate seeds from scratch
use crate::global_definition::*;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Alphanumeric;
use rand::Rng;

/********************
 * Pure Random Generation
 ********************/

/// generate a random number with a range according to the type
/// supported type: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,f32,f64...
pub fn generate_random_number_with_range<T>(lower: T, upper: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    let mut rng = rand::thread_rng();
    let res: T = rng.gen_range(lower..upper);
    res
}

/// generate a random [Bool] value
pub fn generate_random_bool() -> bool {
    let mut rng = rand::thread_rng();
    let res: bool = rng.gen::<bool>();
    res
}

// get max/min value of unsigned number for different languages
fn get_unsigned_edge_value(language: String, size: usize) -> (u128, u128) {
    let max_val: u128;
    let min_val: u128 = MIN_UINT;
    match language.to_ascii_lowercase().as_str() {
        "rust" => {
            match size {
                8 => {
                    max_val = RUST_MAX_U8;
                }
                16 => {
                    max_val = RUST_MAX_U16;
                }
                32 => {
                    max_val = RUST_MAX_U32;
                }
                64 => {
                    max_val = RUST_MAX_U64;
                }
                128 => {
                    max_val = RUST_MAX_U128;
                }
                _ => {
                    max_val = RUST_MAX_U128;
                }
            }
            (max_val, min_val)
        }
        "c" => {
            match size {
                8 => {
                    max_val = C_MAX_UINT8;
                }
                16 => {
                    max_val = C_MAX_USHORT_UINT16;
                }
                32 => {
                    max_val = C_MAX_UINT_ULONG_UINT32;
                }
                64 => {
                    max_val = C_MAX_ULONGLONG_UINT64;
                }
                _ => {
                    max_val = C_MAX_ULONGLONG_UINT64;
                }
            }
            (max_val, min_val)
        }
        "go" => {
            match size {
                8 => {
                    max_val = GO_MAX_UINT8;
                }
                16 => {
                    max_val = GO_MAX_UINT16;
                }
                32 => {
                    max_val = GO_MAX_UINT32;
                }
                64 => {
                    max_val = GO_MAX_UINT64;
                }
                _ => {
                    max_val = GO_MAX_UINT64;
                }
            }
            (max_val, min_val)
        }
        _ => {
            panic!("unsupported language type: {}", language);
        }
    }
}

// get max/min value of signed number for different languages
fn get_signed_edge_value(language: String, size: usize) -> (i128, i128) {
    let max_val: i128;
    let min_val: i128;
    match language.to_ascii_lowercase().as_str() {
        "rust" => {
            match size {
                8 => {
                    max_val = RUST_MAX_I8;
                    min_val = RUST_MIN_I8;
                }
                16 => {
                    max_val = RUST_MAX_I16;
                    min_val = RUST_MIN_I16;
                }
                32 => {
                    max_val = RUST_MAX_I32;
                    min_val = RUST_MIN_I32;
                }
                64 => {
                    max_val = RUST_MAX_I64;
                    min_val = RUST_MIN_I64;
                }
                128 => {
                    max_val = RUST_MAX_I128;
                    min_val = RUST_MIN_I128;
                }
                _ => {
                    max_val = RUST_MAX_I128;
                    min_val = RUST_MIN_I128;
                }
            }
            (max_val, min_val)
        }
        "c" => {
            match size {
                8 => {
                    max_val = C_MAX_INT8;
                    min_val = C_MIN_INT8;
                }
                16 => {
                    max_val = C_MAX_SHORT_INT16;
                    min_val = C_MIN_SHORT_INT16;
                }
                32 => {
                    max_val = C_MAX_INT_LONG_INT32;
                    min_val = C_MIN_INT_LONG_INT32;
                }
                64 => {
                    max_val = C_MAX_LONGLONG_INT64;
                    min_val = C_MIN_LONGLONG_INT64;
                }
                _ => {
                    max_val = C_MAX_LONGLONG_INT64;
                    min_val = C_MIN_LONGLONG_INT64;
                }
            }
            (max_val, min_val)
        }
        "go" => {
            match size {
                8 => {
                    max_val = GO_MAX_INT8;
                    min_val = GO_MIN_INT8;
                }
                16 => {
                    max_val = GO_MAX_INT16;
                    min_val = GO_MIN_INT16;
                }
                32 => {
                    max_val = GO_MAX_INT32;
                    min_val = GO_MIN_INT32;
                }
                64 => {
                    max_val = GO_MAX_INT64;
                    min_val = GO_MIN_INT64;
                }
                _ => {
                    max_val = GO_MAX_INT64;
                    min_val = GO_MIN_INT64;
                }
            }
            (max_val, min_val)
        }
        _ => {
            panic!("unsupported language type: {}", language);
        }
    }
}

/// generate a random unsigned [Number] for Rust
pub fn generate_random_unsigned_number_for_rust(size: usize) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_unsigned_edge_value("rust".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random signed [Number] for Rust
pub fn generate_random_signed_number_for_rust(size: usize) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value("rust".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random unsigned [Number] for C
pub fn generate_random_unsigned_number_for_c(size: usize) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_unsigned_edge_value("c".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random signed [Number] for C
pub fn generate_random_signed_number_for_c(size: usize) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value("c".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random unsigned [Number] for Golang
pub fn generate_random_unsigned_number_for_go(size: usize) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_unsigned_edge_value("go".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random signed [Number] for Golang
pub fn generate_random_signed_number_for_go(size: usize) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value("go".to_string(), size);
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random [Byte] slice with given length
/// the output byte contains ASCII codes
pub fn generate_random_byte_with_length(len: usize) -> Vec<u8> {
    let rng = rand::thread_rng();
    let res: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>();
    res.as_bytes().to_vec()
}

/// generate a random [String] with given length
/// the output string contains ASCII letters and numbers: a-z, A-Z and 0-9.
pub fn generate_random_string_with_length(len: usize) -> String {
    let rng = rand::thread_rng();
    let res: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>();
    res
}

/// generate a random [BigNumber] or [Timestamp] with given length
pub fn generate_random_long_number_with_length(len: usize) -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789";
    let res: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect::<String>();
    res
}

/// generate a random [Array]
pub fn generate_random_array() {
    todo!();
}

/********************
 * Pure Random Mutation
 ********************/

/// maximum number of bit selections in a random mutation
pub const MAX_MUTATE_ITER: u8 = 100;

/// random mutation for byte
pub fn random_mutate_byte(byte: Vec<u8>) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mutate_times: u8 = rng.gen_range(1..MAX_MUTATE_ITER + 1);
    let len = byte.len();
    let mut res = byte;
    for _i in 0..mutate_times {
        let pos = rng.gen_range(0..len);
        // randomly pick a u8 number
        res[pos] = generate_random_number_with_range(u8::MIN, u8::MAX);
    }
    res
}

/// random mutation for string
pub fn random_mutate_string(string: String) -> String {
    let mut rng = rand::thread_rng();
    let mutate_times: u8 = rng.gen_range(1..MAX_MUTATE_ITER + 1);
    let len = string.len();
    let mut res = string;
    for _i in 0..mutate_times {
        let pos = rng.gen_range(0..len);
        // randomly pick a character
        let ch: String = rng
            .clone()
            .sample_iter(&Alphanumeric)
            .take(1)
            .map(char::from)
            .collect::<String>();
        // replace the character in pos index to ch
        res.replace_range(pos..pos + 1, &ch);
    }
    res
}

/// random mutation for timestamp or bigNumber
pub fn random_mutate_long_number(long_number: String) -> String {
    let mut rng = rand::thread_rng();
    let mutate_times: u8 = rng.gen_range(1..MAX_MUTATE_ITER + 1);
    let len = long_number.len();
    let mut res = long_number;
    const CHARSET: &[u8] = b"0123456789";
    for _i in 0..mutate_times {
        let pos = rng.gen_range(0..len);
        // randomly pick a number
        let num: String = (0..1)
            .map(|_| {
                let idx = rng.clone().gen_range(0..CHARSET.len());
                char::from(unsafe { *CHARSET.get_unchecked(idx) })
            })
            .collect::<String>();
        // replace the character in pos index to num
        res.replace_range(pos..pos + 1, &num);
    }
    res
}

/// random mutation for array
pub fn random_mutate_array() {
    todo!();
}

/// [Outer function]: bit flip
pub fn bit_flip() {
    todo!();
}

/// [Outer function]: byte flip
pub fn byte_flip() {
    todo!();
}

/********************
 * Specific Mutation Strategies
 ********************/

/********************
 * Chain-Related Operations
 ********************/

/// calculate hash value, with different types of hash algorithms
pub fn calc_hash<T>(_param_list: Vec<T>) {
    todo!();
}

/// calculate signature value, with different types of signature algorithms
pub fn calc_signature<T>(_param_list: Vec<T>) {
    todo!();
}

/********************
 * User-Related Operations
 ********************/

/// allow mutation with user-defined strategies
#[allow(dead_code)]
pub fn custom_mutate() {}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test Pure Random Generation
     */

    #[test]
    fn test_generate_random_number_with_range() {
        let lower: u32 = 10;
        let upper: u32 = 50;
        let n1: u32 = generate_random_number_with_range(lower, upper);
        assert_eq!((n1 >= lower && n1 < upper), true);
        let lower: i64 = -1000;
        let upper: i64 = 5000;
        let n2: i64 = generate_random_number_with_range(lower, upper);
        assert_eq!((n2 >= lower && n2 < upper), true);
        let lower: f32 = 0.09;
        let upper: f32 = 30.0;
        let p1: f32 = 10f32.powf(-(4 as f32));
        let n3: f32 = generate_random_number_with_range(lower, upper);
        assert_eq!((n3 >= (lower + p1) && n3 < (upper + p1)), true);
    }

    #[test]
    fn test_generate_random_bool() {
        let rand_bool: bool = generate_random_bool();
        assert_eq!((rand_bool == true || rand_bool == false), true);
    }

    #[test]
    fn test_generate_random_unsigned_number_for_rust() {}

    #[test]
    fn test_generate_random_signed_number_for_rust() {}

    #[test]
    fn test_generate_random_unsigned_number_for_c() {}

    #[test]
    fn test_generate_random_signed_number_for_c() {}

    #[test]
    fn test_generate_random_unsigned_number_for_go() {}

    #[test]
    fn test_generate_random_signed_number_for_go() {}

    #[test]
    fn test_generate_random_byte_with_length() {
        let rand_str: Vec<u8> = generate_random_byte_with_length(10);
        assert_eq!(rand_str.len(), 10);
    }

    #[test]
    fn test_generate_random_string_with_length() {
        let rand_str: String = generate_random_string_with_length(10);
        assert_eq!(rand_str.len(), 10);
        assert_eq!(rand_str.chars().count(), 10);
    }

    #[test]
    fn test_generate_random_long_number_with_length() {
        let rand_long_number: String = generate_random_long_number_with_length(10);
        assert_eq!(rand_long_number.len(), 10);
        assert_eq!(rand_long_number.chars().all(char::is_numeric), true);
    }

    /*
     * Test Pure Random Mutation
     */

    #[test]
    fn test_random_mutate_byte() {}

    #[test]
    fn test_random_mutate_string() {}

    #[test]
    fn test_random_mutate_long_number() {}

    /*
     * Test Specific Mutation Strategies
     */

    /*
     * Test Chain-Related Operations
     */
}
