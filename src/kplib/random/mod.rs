use openssl::rand::rand_bytes;
use std::cmp;

pub struct Random {}

impl Random {
    /// Return random integer in range [a, b], including both end points.
    pub fn rand_range(min: i32, max: i32) -> Option<i32> {
        let (min, max) = (cmp::min(min, max), cmp::max(min, max));
        let range = (max - min + 1) as u32;
        let mut buf = [0u8; 4];
        let limit = u32::MAX - (u32::MAX % range);

        loop {
            let res = rand_bytes(&mut buf);

            match res {
                Ok(_) => {
                    let num = u32::from_ne_bytes(buf);

                    // Check if the value is within the desired range
                    if num < limit {
                        return Some(((num % range) + min as u32) as i32);
                    }
                }
                Err(e) => {
                    eprintln!("Error generating random number: {}", e);
                    return None;
                }
            }
        }
    }
}
