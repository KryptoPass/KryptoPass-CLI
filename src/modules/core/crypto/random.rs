use std::cmp;

use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;
use hex;
use openssl::rand::{rand_bytes, rand_priv_bytes};

pub struct Random {}

impl Random {
    /// Generate a random byte string.
    pub fn rand_bytes(n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n];
        rand_bytes(&mut buf).expect("Error generating random bytes");
        buf
    }

    /// Generate a private random byte string.
    pub fn rand_priv_bytes(n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n];
        rand_priv_bytes(&mut buf).expect("Error generating private random bytes");
        buf
    }

    /// Generate a random URL-safe text string.
    pub fn token_urlsafe(n: usize) -> String {
        let bytes = Self::rand_priv_bytes(n);

        URL_SAFE.encode(bytes)
    }

    /// Generate a random byte string and return it as hexadecimal.
    pub fn token_hex(n: usize) -> String {
        let bytes = Self::rand_priv_bytes(n);
        hex::encode(bytes)
    }

    /// Generate a random byte string.
    pub fn token_bytes(n: usize) -> Vec<u8> {
        Self::rand_priv_bytes(n)
    }

    /// Return a random element from a vector.
    pub fn choice<T>(items: &[T]) -> Option<&T> {
        if items.is_empty() {
            None
        } else {
            let index = Self::rand_range(0, (items.len() - 1) as i32)? as usize;
            Some(&items[index])
        }
    }

    /// Shuffle a mutable slice randomly
    pub fn shuffle<T>(items: &mut [T]) {
        let len = items.len();
        for _ in 0..len {
            let i = Self::rand_range(0, (len - 1) as i32).unwrap() as usize;
            let j = Self::rand_range(0, (len - 1) as i32).unwrap() as usize;
            items.swap(i, j);
        }
    }

    pub fn rand_range(min: i32, max: i32) -> Option<i32> {
        let (min, max) = (cmp::min(min, max), cmp::max(min, max));
        let range = (max - min + 1) as u32;
        let limit = u32::MAX - (u32::MAX % range);

        loop {
            let res = Self::rand_bytes(4);
            let array: [u8; 4] = res.try_into().unwrap();

            let num = u32::from_ne_bytes(array);

            // Check if the value is within the desired range
            if num < limit {
                return Some(((num % range) + min as u32) as i32);
            }
        }
    }
}
