use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use aes::Aes256;
use rand::RngCore;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};




fn generate_aes256_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    key
}

fn main() {
    let key_bytes = generate_aes256_key();
    let cipher = Aes256Gcm::new(&key_bytes.into());
    let nonce = Nonce::from_slice(b"unique nonce");
    let plaintext = b"Hello, World!";

    // Обработка ошибок через match
    match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(ciphertext) => {
            println!("Зашифрованный текст: {:02x?}", ciphertext);
            
            match cipher.decrypt(nonce, ciphertext.as_ref()) {
                Ok(decrypted) => {
                    println!("Расшифрованный текст: {:?}", String::from_utf8_lossy(&decrypted));
                }
                Err(e) => eprintln!("Ошибка дешифрования: {:?}", e),
            }
        }
        Err(e) => eprintln!("Ошибка шифрования: {:?}", e),
    }

     
    

    let mc = new_magic_crypt!("magickey", 256);

    let secret_string = "This is a secret message.";

    // Шифрование строки и кодирование результата в Base64
    let base64_encrypted = mc.encrypt_str_to_base64(secret_string);
    println!("Зашифрованная строка: {}", base64_encrypted);

    // Дешифрование строки из Base64
    let decrypted_string = mc.decrypt_base64_to_string(&base64_encrypted).unwrap();
    println!("Расшифрованная строка: {}", decrypted_string);

}