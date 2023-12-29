use core::hash::Hasher;

use siphasher::sip;

#[allow(unused)]
pub fn run_hash() {
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
