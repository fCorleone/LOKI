//! Mutator used by LOKI to mutate the existing seeds or generate seeds from scratch
// Random:
// bool
// u32,i32,u64,i64,
// f32,f64
// String
// Strategy:
// BigNumber, string with all numbers
// Integer: only increasing
// Integer: +1,-1,+10,-10
use anyhow::Result;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::Rng;

/// generate a random number according to the type
/// supported type: bool,u32,i32,u64,i64,f32,f64
pub fn generate_random_number<T>() -> Result<T>
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let res: T = rng.gen::<T>();
    Ok(res)
}

/// generate a random number with a range according to the type
/// supported type: u32,i32,u64,i64,f32,f64
pub fn generate_random_number_with_range<T>(lower: T, upper: T) -> Result<T>
where
    T: SampleUniform + PartialOrd,
{
    let mut rng = rand::thread_rng();
    let res: T = rng.gen_range(lower..upper);
    Ok(res)
}

/// generate a random string with given length
/// the output string contains ASCII letters and numbers: a-z, A-Z and 0-9.
pub fn generate_random_string_with_length(len: usize) -> Result<String> {
    let rng = rand::thread_rng();
    let res: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>();
    Ok(res)
}

/// generate a BigNumber element with given length
pub fn generate_random_big_number_with_length(len: usize) -> Result<String> {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789";
    let res: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect::<String>();
    Ok(res)
}

/// generate a random u8 which will always bigger(>=) than the given one
pub fn get_random_bigger_u8(cur: u8) -> Result<u8> {
    let res: u8;
    match cur {
        // if the existing value is the maximum value of u8, no bigger one will be generated
        u8::MAX => {
            res = u8::MAX;
        }
        0 => {
            let mut rng = rand::thread_rng();
            res = rng.gen::<u8>();
        }
        _ => {
            let mut rng = rand::thread_rng();
            // get a random difference from range 0 ~ u8::MAX-cur
            let diff: u8 = rng.gen_range(0..u8::MAX - cur + 1);
            res = diff + cur;
        }
    }
    Ok(res)
}

/// generate a random number which is slightly varied by the given number
pub fn get_slight_vary(cur: u8, diff: u8) -> Result<u8> {
    let res: u8;
    let mut rng = rand::thread_rng();
    // 50% chance to increase or decrease
    let rand_num: u8 = rng.gen_range(0..2);
    match diff {
        1 => {
            res = if rand_num == 0 { cur - 1 } else { cur + 1 };
        }
        10 => {
            res = if rand_num == 0 { cur - 10 } else { cur + 10 };
        }
        _ => {
            // the default remains unchanged
            res = cur;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_number() {
        let n1: bool = generate_random_number().unwrap();
        assert_eq!((n1 == true || n1 == false), true);
        let n2: u32 = generate_random_number().unwrap();
        assert_eq!((n2 >= u32::MIN && n2 < u32::MAX), true);
        let n3: i32 = generate_random_number().unwrap();
        assert_eq!((n3 >= i32::MIN && n3 < i32::MAX), true);
        let n4: u64 = generate_random_number().unwrap();
        assert_eq!((n4 >= u64::MIN && n4 < u64::MAX), true);
        let n5: i64 = generate_random_number().unwrap();
        assert_eq!((n5 >= i64::MIN && n5 < i64::MAX), true);
        let p1: f32 = 10f32.powf(-(4 as f32));
        let n6: f32 = generate_random_number().unwrap();
        assert_eq!((n6 >= p1 && n6 < (f32::MAX + p1)), true);
        let p2: f64 = 10f64.powf(-(4 as f64));
        let n7: f64 = generate_random_number().unwrap();
        assert_eq!((n7 >= p2 && n7 < (f64::MAX + p2)), true);
    }

    #[test]
    fn test_generate_random_number_with_range() {
        let lower: u32 = 10;
        let upper: u32 = 50;
        let n2: u32 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n2 >= lower && n2 < upper), true);
        let lower: i32 = -10;
        let upper: i32 = 50;
        let n3: i32 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n3 >= lower && n3 < upper), true);
        let lower: u64 = 1000;
        let upper: u64 = 5000;
        let n4: u64 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n4 >= lower && n4 < upper), true);
        let lower: i64 = -1000;
        let upper: i64 = 5000;
        let n5: i64 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n5 >= lower && n5 < upper), true);
        let lower: f32 = 0.09;
        let upper: f32 = 30.0;
        let p1: f32 = 10f32.powf(-(4 as f32));
        let n6: f32 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n6 >= (lower + p1) && n6 < (upper + p1)), true);
        let lower: f64 = 100.09;
        let upper: f64 = 500.0;
        let p2: f64 = 10f64.powf(-(4 as f64));
        let n7: f64 = generate_random_number_with_range(lower, upper).unwrap();
        assert_eq!((n7 >= (lower + p2) && n7 < (upper + p2)), true);
    }

    #[test]
    fn test_generate_random_string_with_length() {
        let rand_str: String = generate_random_string_with_length(10).unwrap();
        assert_eq!(rand_str.len(), 10);
        assert_eq!(rand_str.chars().count(), 10);
    }

    #[test]
    fn test_generate_random_big_number_with_length() {
        let rand_big_number: String = generate_random_big_number_with_length(10).unwrap();
        assert_eq!(rand_big_number.len(), 10);
        assert_eq!(rand_big_number.chars().all(char::is_numeric), true);
    }

    #[test]
    fn test_get_random_bigger_u8() {
        assert_eq!(get_random_bigger_u8(10).unwrap() >= 10, true);
        assert_eq!(get_random_bigger_u8(u8::MAX).unwrap(), u8::MAX);
    }

    #[test]
    fn test_get_slight_vary() {
        let cur: u8 = 5;
        let diff: u8 = 1;
        assert_eq!(
            (get_slight_vary(cur, diff).unwrap() == (cur + 1))
                || (get_slight_vary(cur, diff).unwrap() == (cur - 1)),
            true
        );
        let cur: u8 = 50;
        let diff: u8 = 10;
        assert_eq!(
            (get_slight_vary(cur, diff).unwrap() == (cur + 10))
                || (get_slight_vary(cur, diff).unwrap() == (cur - 10)),
            true
        );
        let diff: u8 = 0;
        assert_eq!(get_slight_vary(cur, diff).unwrap(), cur);
    }
}

/// add a mod to calculate hash value, with different types of hash functions
#[allow(dead_code)]
fn hash() {}
