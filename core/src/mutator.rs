//! Mutator used by LOKI to mutate the existing seeds or generate seeds from scratch
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
    let res = rng.gen_range(min_val..max_val + 1);
    res
}

/// generate a random signed [Number]
pub fn generate_random_signed_number(size: usize, language: String) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, min_val) = get_signed_edge_value(language.to_string(), size);
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

/// obtain an unsigned number greater than or equal to the current value
pub fn strictly_increasing_mutate_for_unsigned_number(
    cur: u128,
    size: usize,
    language: String,
) -> u128 {
    let mut rng = rand::thread_rng();
    let (max_val, _min_val) = get_unsigned_edge_value(language.to_string(), size);
    // get a random number range cur ~ max_val
    let res: u128 = rng.gen_range(cur..max_val + 1);
    res
}

/// obtain a signed number greater than or equal to the current value
pub fn strictly_increasing_mutate_for_signed_number(
    cur: i128,
    size: usize,
    language: String,
) -> i128 {
    let mut rng = rand::thread_rng();
    let (max_val, _min_val) = get_signed_edge_value(language.to_string(), size);
    // get a random number range cur ~ max_val
    let res: i128 = rng.gen_range(cur..max_val + 1);
    res
}

/// obtain an unsigned number less than or equal to the current value
pub fn strictly_decreasing_mutate_for_unsigned_number(
    cur: u128,
    size: usize,
    language: String,
) -> u128 {
    let mut rng = rand::thread_rng();
    let (_max_val, min_val) = get_unsigned_edge_value(language.to_string(), size);
    // get a random difference from range min_val ~ cur
    let res: u128 = rng.gen_range(min_val..cur + 1);
    res
}

/// obtain an unsigned number less than or equal to the current value
pub fn strictly_decreasing_mutate_for_signed_number(
    cur: i128,
    size: usize,
    language: String,
) -> i128 {
    let mut rng = rand::thread_rng();
    let (_max_val, min_val) = get_signed_edge_value(language.to_string(), size);
    // get a random difference from range min_val ~ cur
    let res: i128 = rng.gen_range(min_val..cur + 1);
    res
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
    fn test_generate_random_unsigned_number() {}

    #[test]
    fn test_generate_random_signed_number() {}

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

    #[test]
    fn test_strictly_increasing_mutate_for_unsigned_number() {}

    #[test]
    fn test_strictly_increasing_mutate_for_signed_number() {}

    #[test]
    fn test_strictly_decreasing_mutate_for_unsigned_number() {}

    #[test]
    fn test_strictly_decreasing_mutate_for_signed_number() {}

    #[test]
    fn test_fine_tuning_mutate_for_unsigned_number() {}

    #[test]
    fn test_fine_tuning_mutate_for_signed_number() {}

    #[test]
    fn test_fine_tuning_mutate_for_long_number() {
        let mut long_numer: String = "185".to_string();
        let mut margin: u128 = 37;
        let mut ans: String =
            fine_tuning_mutate_for_long_number(long_numer, margin, "+".to_string());
        assert_eq!(ans, "222");

        long_numer = "99".to_string();
        margin = 2;
        ans = fine_tuning_mutate_for_long_number(long_numer, margin, "+".to_string());
        assert_eq!(ans, "101");

        long_numer = "185".to_string();
        margin = 37;
        ans = fine_tuning_mutate_for_long_number(long_numer, margin, "-".to_string());
        assert_eq!(ans, "148");

        long_numer = "1".to_string();
        margin = 113;
        ans = fine_tuning_mutate_for_long_number(long_numer, margin, "-".to_string());
        assert_eq!(ans, "1888");
    }

    #[test]
    fn test_edge_value_mutate_for_unsigned_number() {}

    #[test]
    fn test_edge_value_mutate_for_signed_number() {}

    /*
     * Test Chain-Related Operations
     */
}
