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
use rand::distributions::{Distribution, Standard};
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
            // get a random difference from range 1 ~ u8::MAX-cur+1
            let diff: u8 = rng.gen_range(0..u8::MAX - cur + 1);
            res = diff + cur;
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
        assert_eq!(
            (n1 == true || n1 == false), 
            true
        );
        let n2: u32 = generate_random_number().unwrap();
        assert_eq!(
            (n2 >= u32::MIN && n2 < u32::MAX),
            true
        );
        let n3: i32 = generate_random_number().unwrap();
        assert_eq!(
            (n3 >= i32::MIN && n3 < i32::MAX),
            true
        );
        let n4: u64 = generate_random_number().unwrap();
        assert_eq!(
            (n4 >= u64::MIN && n4 < u64::MAX),
            true
        );
        let n5: i64 = generate_random_number().unwrap();
        assert_eq!(
            (n5 >= i64::MIN && n5 < i64::MAX),
            true
        );
        let p1: f32 = 10f32.powf(-(4 as f32)); 
        let n6: f32 = generate_random_number().unwrap();
        assert_eq!(
            (n6 >= p1 && n6 < (f32::MAX + p1)),
            true
        );
        let p2: f64 = 10f64.powf(-(4 as f64)); 
        let n7: f64 = generate_random_number().unwrap();
        assert_eq!(
            (n7 >= p2 && n7 < (f64::MAX + p2)),
            true
        );
    }

    #[test]
    fn test_get_random_bigger_u8() {
        assert_eq!(
            get_random_bigger_u8(10).unwrap() >= 10,
            true
        );
        assert_eq!(
            get_random_bigger_u8(u8::MAX).unwrap(),
            u8::MAX
        );
    }
}

/// add a mod to calculate hash value, with different types of hash functions
#[allow(dead_code)]
fn hash() {}
