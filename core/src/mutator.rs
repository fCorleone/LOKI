//! Mutator used by LOKI to mutate the existing seeds or generate seeds from scratch
use crate::loki_type::{Array, BasicType};
use crate::utils::*;
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

/// generate a random unsigned [Number]
pub fn generate_random_unsigned_number(size: usize, language: String) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_unsigned_edge_value(language.to_string(), size);
    // get a random number in range min_val ~ max_val
    let mut res = rng.gen_range(min_val..max_val);
    if res == max_val - 1 {
        res = max_val;
    }
    res
}

/// generate a random signed [Number]
pub fn generate_random_signed_number(size: usize, language: String) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value(language.to_string(), size);
    // get a random number in range min_val ~ max_val
    let mut res = rng.gen_range(min_val..max_val);
    if res == max_val - 1 {
        res = max_val;
    }
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
pub fn generate_random_array() -> Array {
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

/// random mutation for array, here we randomly mutate the lenth as well as the content
pub fn random_mutate_array(original_arr: &mut Array) -> Array {
    if original_arr.get_length() == 0 {
        return generate_random_array();
    }
    let mut _rng = rand::thread_rng();
    let old_len = original_arr.get_length();
    let new_len = mutate_array_len(old_len);
    let mut _shuffled = false;
    let ele_type = original_arr.get_ele_ty();
    if new_len > old_len {
        match ele_type {
            BasicType::BIGNUMBER => {
                let new_vals = (0..new_len - old_len)
                    .map(|_| generate_random_long_number_with_length(1000))
                    .collect::<Vec<_>>();
                original_arr.get_mut_content().extend(new_vals);
            }
            _ => {}
        }
    } else if new_len < old_len {
        // delete some elements
    } else {
        // shuffle the original array
    }
    debug_info!("mutate_array: {} -> {}", old_len, new_len);

    todo!()
}

/// random choose a new len for an array
pub fn mutate_array_len(old_len: u32) -> u32 {
    let mut new_len = old_len;
    let mut rng = rand::thread_rng();
    if rng.gen() {
        while new_len == old_len || rng.gen() {
            new_len += 1;
        }
    } else {
        new_len = rng.gen_range(0..=10)
    };
    new_len
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

/// obtain an unsigned number greater than or equal to the current value
pub fn strictly_increasing_mutate_for_unsigned_number(
    cur: u128,
    size: usize,
    language: String,
) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, _min_val) = get_unsigned_edge_value(language.to_string(), size);
    if cur == max_val {
        return cur;
    }
    // get a random number in range cur ~ max_val
    let mut res: u128 = rng.gen_range(cur..max_val);
    if res == max_val - 1 {
        res = max_val;
    }
    return res;
}

/// obtain a signed number greater than or equal to the current value
pub fn strictly_increasing_mutate_for_signed_number(
    cur: i128,
    size: usize,
    language: String,
) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, _min_val) = get_signed_edge_value(language.to_string(), size);
    if cur == max_val {
        return cur;
    }
    // get a random number in range cur ~ max_val
    let mut res: i128 = rng.gen_range(cur..max_val);
    if res == max_val - 1 {
        res = max_val;
    }
    return res;
}

/// obtain an unsigned number less than or equal to the current value
pub fn strictly_decreasing_mutate_for_unsigned_number(
    cur: u128,
    size: usize,
    language: String,
) -> u128 {
    let mut rng = rand::thread_rng();
    let (_max_val, min_val) = get_unsigned_edge_value(language.to_string(), size);
    if cur == min_val {
        return cur;
    }
    // get a random difference in range min_val ~ cur
    let res: u128 = rng.gen_range(min_val..cur + 1);
    return res;
}

/// obtain an unsigned number less than or equal to the current value
pub fn strictly_decreasing_mutate_for_signed_number(
    cur: i128,
    size: usize,
    language: String,
) -> i128 {
    let mut rng = rand::thread_rng();
    let (_max_val, min_val) = get_signed_edge_value(language.to_string(), size);
    if cur == min_val {
        return cur;
    }
    // get a random difference in range min_val ~ cur
    let res: i128 = rng.gen_range(min_val..cur + 1);
    return res;
}

/// fine-tuning for unsigned numbers
pub fn fine_tuning_mutate_for_unsigned_number(cur: u128, margin: u128, op: String) -> u128 {
    let mut rng = rand::thread_rng();
    match op.as_str() {
        "+" => {
            return cur + margin;
        }
        "-" => {
            return cur - margin;
        }
        _ => {
            // randomly choose to increase or decrease
            let rand: u8 = rng.gen_range(0..2);
            match rand {
                0 => {
                    return cur + margin;
                }
                1 => {
                    return cur - margin;
                }
                _ => {
                    return 0;
                }
            }
        }
    }
}

/// fine-tuning for signed numbers
pub fn fine_tuning_mutate_for_signed_number(cur: i128, margin: i128, op: String) -> i128 {
    let mut rng = rand::thread_rng();
    match op.as_str() {
        "+" => {
            return cur + margin;
        }
        "-" => {
            return cur - margin;
        }
        _ => {
            // randomly choose to increase or decrease
            let rand: u8 = rng.gen_range(0..2);
            match rand {
                0 => {
                    return cur + margin;
                }
                1 => {
                    return cur - margin;
                }
                _ => {
                    return 0;
                }
            }
        }
    }
}

/// fine-tuning for long numbers (bigNumber or timestamp)
pub fn fine_tuning_mutate_for_long_number(long_number: String, margin: u128, op: String) -> String {
    let mut rng = rand::thread_rng();
    match op.as_str() {
        "+" => {
            return add_string_by_bits(long_number, margin.to_string());
        }
        "-" => {
            return sub_string_by_bits(long_number, margin.to_string());
        }
        _ => {
            // randomly choose to increase or decrease
            let rand: u8 = rng.gen_range(0..2);
            match rand {
                0 => {
                    return add_string_by_bits(long_number, margin.to_string());
                }
                1 => {
                    return sub_string_by_bits(long_number, margin.to_string());
                }
                _ => {
                    return "0".to_string();
                }
            }
        }
    }
}

/// obtain the max/min unsigned value of corresponding type
pub fn edge_value_mutate_for_unsigned_number(size: usize, language: String) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_unsigned_edge_value(language.to_string(), size);
    // randomly select the maximum or minimum value to return
    let choice: u8 = rng.gen_range(0..2);
    match choice {
        0 => {
            return max_val;
        }
        1 => {
            return min_val;
        }
        _ => {
            return 0;
        }
    }
}

/// obtain the max/min signed value of corresponding type
pub fn edge_value_mutate_for_signed_number(size: usize, language: String) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value(language.to_string(), size);
    // randomly select the maximum or minimum value to return
    let choice: u8 = rng.gen_range(0..2);
    match choice {
        0 => {
            return max_val;
        }
        1 => {
            return min_val;
        }
        _ => {
            return 0;
        }
    }
}

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
    use crate::global_definition::*;

    /*
     * Test Pure Random Generation
     */

    #[test]
    fn test_generate_random_number_with_range() {
        let lower: u32 = 10;
        let upper: u32 = 50;
        let n1: u32 = generate_random_number_with_range(lower, upper);
        assert!(n1 >= lower && n1 < upper);
        let lower: i64 = -1000;
        let upper: i64 = 5000;
        let n2: i64 = generate_random_number_with_range(lower, upper);
        assert!(n2 >= lower && n2 < upper);
        let lower: f32 = 0.09;
        let upper: f32 = 30.0;
        let p1: f32 = 10f32.powf(-(4 as f32));
        let n3: f32 = generate_random_number_with_range(lower, upper);
        assert!(n3 >= (lower + p1) && n3 < (upper + p1));
    }

    #[test]
    fn test_generate_random_bool() {
        let rand_bool: bool = generate_random_bool();
        assert!(rand_bool == true || rand_bool == false);
    }

    #[test]
    fn test_generate_random_unsigned_number() {
        let mut n = generate_random_unsigned_number(8, "Rust".to_string());
        assert!(n <= RUST_MAX_U8);
        n = generate_random_unsigned_number(16, "Rust".to_string());
        assert!(n <= RUST_MAX_U16);
        n = generate_random_unsigned_number(32, "Rust".to_string());
        assert!(n <= RUST_MAX_U32);
        n = generate_random_unsigned_number(64, "Rust".to_string());
        assert!(n <= RUST_MAX_U64);
        n = generate_random_unsigned_number(128, "Rust".to_string());
        assert!(n <= RUST_MAX_U128);

        n = generate_random_unsigned_number(8, "C".to_string());
        assert!(n <= C_MAX_UINT8);
        n = generate_random_unsigned_number(16, "C".to_string());
        assert!(n <= C_MAX_USHORT_UINT16);
        n = generate_random_unsigned_number(32, "C".to_string());
        assert!(n <= C_MAX_UINT_ULONG_UINT32);
        n = generate_random_unsigned_number(64, "C".to_string());
        assert!(n <= C_MAX_ULONGLONG_UINT64);

        n = generate_random_unsigned_number(8, "Go".to_string());
        assert!(n <= GO_MAX_UINT8);
        n = generate_random_unsigned_number(16, "Go".to_string());
        assert!(n <= GO_MAX_UINT16);
        n = generate_random_unsigned_number(32, "Go".to_string());
        assert!(n <= GO_MAX_UINT32);
        n = generate_random_unsigned_number(64, "Go".to_string());
        assert!(n <= GO_MAX_UINT64);
    }

    #[test]
    fn test_generate_random_signed_number() {
        let mut n = generate_random_signed_number(8, "Rust".to_string());
        assert!(n >= RUST_MIN_I8 && n <= RUST_MAX_I8);
        n = generate_random_signed_number(16, "Rust".to_string());
        assert!(n >= RUST_MIN_I16 && n <= RUST_MAX_I16);
        n = generate_random_signed_number(32, "Rust".to_string());
        assert!(n >= RUST_MIN_I32 && n <= RUST_MAX_I32);
        n = generate_random_signed_number(64, "Rust".to_string());
        assert!(n >= RUST_MIN_I64 && n <= RUST_MAX_I64);
        n = generate_random_signed_number(128, "Rust".to_string());
        assert!(n >= RUST_MIN_I128 && n <= RUST_MAX_I128);

        n = generate_random_signed_number(8, "C".to_string());
        assert!(n >= C_MIN_INT8 && n <= C_MAX_INT8);
        n = generate_random_signed_number(16, "C".to_string());
        assert!(n >= C_MIN_SHORT_INT16 && n <= C_MAX_SHORT_INT16);
        n = generate_random_signed_number(32, "C".to_string());
        assert!(n >= C_MIN_INT_LONG_INT32 && n <= C_MAX_INT_LONG_INT32);
        n = generate_random_signed_number(64, "C".to_string());
        assert!(n >= C_MIN_LONGLONG_INT64 && n <= C_MAX_LONGLONG_INT64);

        n = generate_random_signed_number(8, "Go".to_string());
        assert!(n >= GO_MIN_INT8 && n <= GO_MAX_INT8);
        n = generate_random_signed_number(16, "Go".to_string());
        assert!(n >= GO_MIN_INT16 && n <= GO_MAX_INT16);
        n = generate_random_signed_number(32, "Go".to_string());
        assert!(n >= GO_MIN_INT32 && n <= GO_MAX_INT32);
        n = generate_random_signed_number(64, "Go".to_string());
        assert!(n >= GO_MIN_INT64 && n <= GO_MAX_INT64);
    }

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
        assert!(rand_long_number.chars().all(char::is_numeric));
    }

    /*
     * Test Pure Random Mutation
     */

    #[test]
    fn test_random_mutate_byte() {
        let rand_str: Vec<u8> = generate_random_byte_with_length(10);
        let mut_rand_str: Vec<u8> = random_mutate_byte(rand_str.clone());
        assert_eq!(mut_rand_str.len(), 10);
        assert_ne!(mut_rand_str, rand_str);
    }

    #[test]
    fn test_random_mutate_string() {
        let rand_str: String = generate_random_string_with_length(10);
        let mut_rand_str: String = random_mutate_string(rand_str.clone());
        assert_eq!(mut_rand_str.len(), 10);
        assert_eq!(mut_rand_str.chars().count(), 10);
    }

    #[test]
    fn test_random_mutate_long_number() {
        let rand_long_number: String = generate_random_long_number_with_length(10);
        let mut_rand_long_number: String = random_mutate_long_number(rand_long_number.clone());
        assert_eq!(mut_rand_long_number.len(), 10);
        assert!(mut_rand_long_number.chars().all(char::is_numeric));
    }

    /*
     * Test Specific Mutation Strategies
     */

    #[test]
    fn test_strictly_increasing_mutate_for_unsigned_number() {
        let mut cur: u128 = 217;
        let mut mut_num =
            strictly_increasing_mutate_for_unsigned_number(cur, 8, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_U8);
        cur = 256;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 16, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_U16);
        cur = 63357;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 32, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_U32);
        cur = 4294967299;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 64, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_U64);
        cur = 18446744073709551651;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 128, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_U128);

        cur = 255;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 8, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_UINT8);
        cur = 65534;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 16, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_USHORT_UINT16);
        cur = 4294967292;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 32, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_UINT_ULONG_UINT32);
        cur = 18446744073709551610;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 64, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_ULONGLONG_UINT64);

        cur = 251;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 8, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_UINT8);
        cur = 65534;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 16, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_UINT16);
        cur = 4294967295;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 32, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_UINT32);
        cur = 18446744073709551610;
        mut_num = strictly_increasing_mutate_for_unsigned_number(cur, 64, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_UINT64);
    }

    #[test]
    fn test_strictly_increasing_mutate_for_signed_number() {
        let mut cur: i128 = 9;
        let mut mut_num = strictly_increasing_mutate_for_signed_number(cur, 8, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_I8);
        cur = -32766;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 16, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_I16);
        cur = 2147483644;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 32, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_I32);
        cur = -9223372036854775801;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 64, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_I64);
        cur = 170141183460469231731687303715884105725;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 128, "rust".to_string());
        assert!(mut_num >= cur && mut_num <= RUST_MAX_I128);

        cur = 125;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 8, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_INT8);
        cur = -32767;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 16, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_SHORT_INT16);
        cur = 21474836;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 32, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_INT_LONG_INT32);
        cur = -9223372036854775;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 64, "c".to_string());
        assert!(mut_num >= cur && mut_num <= C_MAX_LONGLONG_INT64);

        cur = 125;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 8, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_INT8);
        cur = -32767;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 16, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_INT16);
        cur = 21474836;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 32, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_INT32);
        cur = -9223372036854775;
        mut_num = strictly_increasing_mutate_for_signed_number(cur, 64, "go".to_string());
        assert!(mut_num >= cur && mut_num <= GO_MAX_INT64);
    }

    #[test]
    fn test_strictly_decreasing_mutate_for_unsigned_number() {
        let mut cur: u128 = 217;
        let mut mut_num =
            strictly_decreasing_mutate_for_unsigned_number(cur, 8, "rust".to_string());
        assert!(mut_num <= cur);
        cur = 256;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 16, "rust".to_string());
        assert!(mut_num <= cur);
        cur = 63357;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 32, "rust".to_string());
        assert!(mut_num <= cur);
        cur = 4294967299;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 64, "rust".to_string());
        assert!(mut_num <= cur);
        cur = 18446744073709551651;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 128, "rust".to_string());
        assert!(mut_num <= cur);

        cur = 255;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 8, "c".to_string());
        assert!(mut_num <= cur);
        cur = 65534;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 16, "c".to_string());
        assert!(mut_num <= cur);
        cur = 4294967292;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 32, "c".to_string());
        assert!(mut_num <= cur);
        cur = 18446744073709551610;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 64, "c".to_string());
        assert!(mut_num <= cur);

        cur = 251;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 8, "go".to_string());
        assert!(mut_num <= cur);
        cur = 65534;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 16, "go".to_string());
        assert!(mut_num <= cur);
        cur = 4294967295;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 32, "go".to_string());
        assert!(mut_num <= cur);
        cur = 18446744073709551610;
        mut_num = strictly_decreasing_mutate_for_unsigned_number(cur, 64, "go".to_string());
        assert!(mut_num <= cur);
    }

    #[test]
    fn test_strictly_decreasing_mutate_for_signed_number() {
        let mut cur: i128 = 9;
        let mut mut_num = strictly_decreasing_mutate_for_signed_number(cur, 8, "rust".to_string());
        assert!(mut_num >= RUST_MIN_I8 && mut_num <= cur);
        cur = -32766;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 16, "rust".to_string());
        assert!(mut_num >= RUST_MIN_I16 && mut_num <= cur);
        cur = 2147483644;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 32, "rust".to_string());
        assert!(mut_num >= RUST_MIN_I32 && mut_num <= cur);
        cur = -9223372036854775801;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 64, "rust".to_string());
        assert!(mut_num >= RUST_MIN_I64 && mut_num <= cur);
        cur = 170141183460469231731687303715884105725;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 128, "rust".to_string());
        assert!(mut_num >= RUST_MIN_I128 && mut_num <= cur);

        cur = 125;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 8, "c".to_string());
        assert!(mut_num >= C_MIN_INT8 && mut_num <= cur);
        cur = -32767;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 16, "c".to_string());
        assert!(mut_num >= C_MIN_SHORT_INT16 && mut_num <= cur);
        cur = 21474836;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 32, "c".to_string());
        assert!(mut_num >= C_MIN_INT_LONG_INT32 && mut_num <= cur);
        cur = -9223372036854775;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 64, "c".to_string());
        assert!(mut_num >= C_MIN_LONGLONG_INT64 && mut_num <= cur);

        cur = 125;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 8, "go".to_string());
        assert!(mut_num >= GO_MIN_INT8 && mut_num <= cur);
        cur = -32767;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 16, "go".to_string());
        assert!(mut_num >= GO_MIN_INT16 && mut_num <= cur);
        cur = 21474836;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 32, "go".to_string());
        assert!(mut_num >= GO_MIN_INT32 && mut_num <= cur);
        cur = -9223372036854775;
        mut_num = strictly_decreasing_mutate_for_signed_number(cur, 64, "go".to_string());
        assert!(mut_num >= GO_MIN_INT64 && mut_num <= cur);
    }

    #[test]
    fn test_fine_tuning_mutate_for_unsigned_number() {
        let cur: u128 = 15;
        let margin: u128 = 1;
        let ft1 = fine_tuning_mutate_for_unsigned_number(cur, margin, "+".to_string());
        assert_eq!(ft1, 16);
        let ft2 = fine_tuning_mutate_for_unsigned_number(cur, margin, "-".to_string());
        assert_eq!(ft2, 14);
        let ft3 = fine_tuning_mutate_for_unsigned_number(cur, margin, "*".to_string());
        assert!(ft3 == 16 || ft3 == 14);
    }

    #[test]
    fn test_fine_tuning_mutate_for_signed_number() {
        let cur: i128 = -10;
        let margin: i128 = 1;
        let ft1 = fine_tuning_mutate_for_signed_number(cur, margin, "+".to_string());
        assert_eq!(ft1, -9);
        let ft2 = fine_tuning_mutate_for_signed_number(cur, margin, "-".to_string());
        assert_eq!(ft2, -11);
        let ft3 = fine_tuning_mutate_for_signed_number(cur, margin, "*".to_string());
        assert!(ft3 == -11 || ft3 == -9);
    }

    #[test]
    fn test_fine_tuning_mutate_for_long_number() {
        let mut long_number: String = "185".to_string();
        let mut margin: u128 = 37;
        let mut ans: String =
            fine_tuning_mutate_for_long_number(long_number, margin, "+".to_string());
        assert_eq!(ans, "222");

        long_number = "99".to_string();
        margin = 2;
        ans = fine_tuning_mutate_for_long_number(long_number, margin, "+".to_string());
        assert_eq!(ans, "101");

        long_number = "185".to_string();
        margin = 37;
        ans = fine_tuning_mutate_for_long_number(long_number, margin, "-".to_string());
        assert_eq!(ans, "148");

        long_number = "10000000000000".to_string();
        margin = 10000000000002;
        ans = fine_tuning_mutate_for_long_number(long_number, margin, "-".to_string());
        assert_eq!(ans, "199999999999998");
    }

    #[test]
    fn test_edge_value_mutate_for_unsigned_number() {
        let mut ev = edge_value_mutate_for_unsigned_number(8, "Rust".to_string());
        assert!(ev == 0 || ev == RUST_MAX_U8);
        ev = edge_value_mutate_for_unsigned_number(16, "Rust".to_string());
        assert!(ev == 0 || ev <= RUST_MAX_U16);
        ev = edge_value_mutate_for_unsigned_number(32, "Rust".to_string());
        assert!(ev == 0 || ev <= RUST_MAX_U32);
        ev = edge_value_mutate_for_unsigned_number(64, "Rust".to_string());
        assert!(ev == 0 || ev <= RUST_MAX_U64);
        ev = edge_value_mutate_for_unsigned_number(128, "Rust".to_string());
        assert!(ev == 0 || ev <= RUST_MAX_U128);

        ev = edge_value_mutate_for_unsigned_number(8, "C".to_string());
        assert!(ev == 0 || ev <= C_MAX_UINT8);
        ev = edge_value_mutate_for_unsigned_number(16, "C".to_string());
        assert!(ev == 0 || ev <= C_MAX_USHORT_UINT16);
        ev = edge_value_mutate_for_unsigned_number(32, "C".to_string());
        assert!(ev == 0 || ev <= C_MAX_UINT_ULONG_UINT32);
        ev = edge_value_mutate_for_unsigned_number(64, "C".to_string());
        assert!(ev == 0 || ev <= C_MAX_ULONGLONG_UINT64);

        ev = edge_value_mutate_for_unsigned_number(8, "Go".to_string());
        assert!(ev == 0 || ev <= GO_MAX_UINT8);
        ev = edge_value_mutate_for_unsigned_number(16, "Go".to_string());
        assert!(ev == 0 || ev <= GO_MAX_UINT16);
        ev = edge_value_mutate_for_unsigned_number(32, "Go".to_string());
        assert!(ev == 0 || ev <= GO_MAX_UINT32);
        ev = edge_value_mutate_for_unsigned_number(64, "Go".to_string());
        assert!(ev == 0 || ev <= GO_MAX_UINT64);
    }

    #[test]
    fn test_edge_value_mutate_for_signed_number() {
        let mut ev = edge_value_mutate_for_signed_number(8, "Rust".to_string());
        assert!(ev == RUST_MIN_I8 || ev == RUST_MAX_I8);
        ev = edge_value_mutate_for_signed_number(16, "Rust".to_string());
        assert!(ev == RUST_MIN_I16 || ev == RUST_MAX_I16);
        ev = edge_value_mutate_for_signed_number(32, "Rust".to_string());
        assert!(ev == RUST_MIN_I32 || ev == RUST_MAX_I32);
        ev = edge_value_mutate_for_signed_number(64, "Rust".to_string());
        assert!(ev == RUST_MIN_I64 || ev == RUST_MAX_I64);
        ev = edge_value_mutate_for_signed_number(128, "Rust".to_string());
        assert!(ev == RUST_MIN_I128 || ev == RUST_MAX_I128);

        ev = edge_value_mutate_for_signed_number(8, "C".to_string());
        assert!(ev == C_MIN_INT8 || ev == C_MAX_INT8);
        ev = edge_value_mutate_for_signed_number(16, "C".to_string());
        assert!(ev == C_MIN_SHORT_INT16 || ev == C_MAX_SHORT_INT16);
        ev = edge_value_mutate_for_signed_number(32, "C".to_string());
        assert!(ev == C_MIN_INT_LONG_INT32 || ev == C_MAX_INT_LONG_INT32);
        ev = edge_value_mutate_for_signed_number(64, "C".to_string());
        assert!(ev == C_MIN_LONGLONG_INT64 || ev == C_MAX_LONGLONG_INT64);

        ev = edge_value_mutate_for_signed_number(8, "Go".to_string());
        assert!(ev == GO_MIN_INT8 || ev == GO_MAX_INT8);
        ev = edge_value_mutate_for_signed_number(16, "Go".to_string());
        assert!(ev == GO_MIN_INT16 || ev == GO_MAX_INT16);
        ev = edge_value_mutate_for_signed_number(32, "Go".to_string());
        assert!(ev == GO_MIN_INT32 || ev == GO_MAX_INT32);
        ev = edge_value_mutate_for_signed_number(64, "Go".to_string());
        assert!(ev == GO_MIN_INT64 || ev == GO_MAX_INT64);
    }

    /*
     * Test Chain-Related Operations
     */
}
