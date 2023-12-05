use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use cfb_mode::Encryptor;
use cfb_mode::Decryptor;
use aes::Aes128;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    type Aes128CfbEnc = Encryptor<Aes128>;
    let mut buf = plaintext.to_vec();
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut buf);
    buf
}

#[wasm_bindgen]
pub fn decrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    type Aes128CfbDec = Decryptor<Aes128>;
    let mut buf = plaintext.to_vec();
    Aes128CfbDec::new(key.into(), iv.into()).decrypt(&mut buf);
    buf
}