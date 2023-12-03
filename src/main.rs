extern crate aes;
extern crate block_modes;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cfb};

type AesCfb = Cfb<Aes128, Pkcs7>;

#[no_mangle]
pub extern "C" fn encrypt_data(data: *const u8, data_len: usize, key: *const u8, key_len: usize, result: *mut u8) {
    // Convert the raw pointers to slices
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };

    // AES encryption logic here
    let cipher = AesCfb::new_var(key, &[0u8; 16]).unwrap();
    let ciphertext = cipher.encrypt_vec(data);

    // Copy the encrypted data to the result pointer
    unsafe {
        std::ptr::copy(ciphertext.as_ptr(), result, ciphertext.len());
    }
}

#[no_mangle]
pub extern "C" fn decrypt_data(data: *const u8, data_len: usize, key: *const u8, key_len: usize, result: *mut u8) {
    // Convert the raw pointers to slices
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };

    // AES decryption logic here
    let cipher = AesCfb::new_var(key, &[0u8; 16]).unwrap();
    let plaintext = cipher.decrypt_vec(data);

    // Copy the decrypted data to the result pointer
    unsafe {
        std::ptr::copy(plaintext.as_ptr(), result, plaintext.len());
    }
}
