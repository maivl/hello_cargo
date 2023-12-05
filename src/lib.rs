use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use cfb_mode::Encryptor;
use cfb_mode::Decryptor;
use aes::Aes128;
use arrayref::array_ref;

#[no_mangle]
pub fn encrypt(key: Vec<u8>, iv: Vec<u8>, plaintext: &[u8]) -> Vec<u8> {
    type Aes128CfbEnc = Encryptor<Aes128>;
    let mut buf = plaintext.to_vec();
    let key_slice: &[u8; 16] = array_ref!(key, 0, 16);
    let iv_slice: &[u8; 16] = array_ref!(iv, 0, 16);
    Aes128CfbEnc::new(key_slice.into(), iv_slice.into()).encrypt(&mut buf);
    buf
}

#[no_mangle]
pub fn decrypt(key: Vec<u8>, iv: Vec<u8>, ciphertext: &[u8]) -> Vec<u8> {
    type Aes128CfbDec = Decryptor<Aes128>;
    let mut buf = ciphertext.to_vec();
    let key_slice: &[u8; 16] = array_ref!(key, 0, 16);
    let iv_slice: &[u8; 16] = array_ref!(iv, 0, 16);
    Aes128CfbDec::new(key_slice.into(), iv_slice.into()).decrypt(&mut buf);
    buf
}