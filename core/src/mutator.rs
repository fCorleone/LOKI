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
use rand::Rng;

/// generate a random bool
pub fn random_bool() -> Result<bool> {
    let mut rng = rand::thread_rng();
    let res: bool = rng.gen();
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
    #[test]
    fn bigger_u8_works() {
        assert_eq!(
            crate::mutator::get_random_bigger_u8(10).unwrap() >= 10,
            true
        );
        assert_eq!(
            crate::mutator::get_random_bigger_u8(u8::MAX).unwrap(),
            u8::MAX
        );
    }
}

/// add a mod to calculate hash value, with different types of hash functions
#[allow(dead_code)]
fn hash() {}
