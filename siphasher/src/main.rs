// 这段代码的含义是，如果不在测试模式下，就启用 no_std 模式，否则忽略 no_std。
// #![cfg_attr(not(test), no_std)]

#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::many_single_char_names)]

use core::hash::Hasher;

pub mod sip;
pub mod sip128;

#[cfg(any(feature = "serde", feature = "serde_std", feature = "serde_no_std"))]
pub mod reexports {
    pub use serde;
    #[cfg(feature = "serde_json")]
    pub use serde_json;
}

pub mod prelude {
    pub use core::hash::Hasher as _;

    pub use sip128::Hasher128 as _;

    pub use crate::sip128;
}

fn main() {
    let array: &[u8] = &[1, 2, 3];
    let key: &[u8; 16] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let hasher = sip::SipHasher13::new_with_key(key);
    let h = hasher.hash(array);
    println!("the h is {h}");

    let array1: &[u8] = &[1, 2, 3];
    let array2: &[u8] = &[4, 5, 6];
    let key: &[u8; 16] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut hasher = sip::SipHasher13::new_with_key(key);
    hasher.write(array1);
    hasher.write(array2);
    let h = hasher.finish();
    println!("the h is {h}");
}
