use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use hex_literal::hex;

type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;
type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;


pub fn main () {

    let key = [0x42; 16];
    let iv = [0x24; 16];
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hex!(
        "3357121ebb5a29468bd861467596ce3d6f99e251cc2d9f0a598032ae386d0ab995b3"
    );

    // encrypt/decrypt in-place
    let mut buf = plaintext.to_vec();
    Aes128CfbEnc::new(&key.into(), &iv.into()).encrypt(&mut buf);
    assert_eq!(buf[..], ciphertext[..]);

    Aes128CfbDec::new(&key.into(), &iv.into()).decrypt(&mut buf);
    assert_eq!(buf[..], plaintext[..]);

    // encrypt/decrypt from buffer to buffer
    // buffer length must be equal to input length
    let mut buf1 = [0u8; 34];
    Aes128CfbEnc::new(&key.into(), &iv.into())
        .encrypt_b2b(&plaintext, &mut buf1)
        .unwrap();
    assert_eq!(buf1[..], ciphertext[..]);

    let mut buf2 = [0u8; 34];
    Aes128CfbDec::new(&key.into(), &iv.into())
        .decrypt_b2b(&buf1, &mut buf2)
        .unwrap();
    assert_eq!(buf2[..], plaintext[..]);
}