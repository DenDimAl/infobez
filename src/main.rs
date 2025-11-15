use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs8::{LineEnding, EncodePrivateKey, EncodePublicKey}, Oaep};
use sha2::{Sha256, Digest};
use rsa::signature::{RandomizedSigner, Verifier};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let private_key_pem = priv_key.to_pkcs8_pem(LineEnding::LF)?;
    let public_key_pem = pub_key.to_public_key_pem(LineEnding::LF)?;
    
    
    std::fs::write("private_key.pem", private_key_pem.as_bytes())?;
    std::fs::write("public_key.pem", public_key_pem.as_bytes())?;
    
     let message = b"Hello, RSA!";
    println!("\nИсходное сообщение: {}", String::from_utf8_lossy(message));
    
    let encrypted = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, message)?;
    println!("Зашифрованное сообщение: {:?}", encrypted);
    
    let decrypted = priv_key.decrypt(Pkcs1v15Encrypt, &encrypted)?;
    println!("Расшифрованное сообщение: {}", String::from_utf8_lossy(&decrypted));

    let data = b"Important data to sign";
    println!("\nДанные для подписи: {}", String::from_utf8_lossy(data));
    
    let padding_encrypt = Oaep::new::<Sha256>();
    let encrypted = pub_key.encrypt(&mut rng, padding_encrypt, data)?;
    println!("Зашифрованные данные: {} байт", encrypted.len());
    
    let padding_decrypt = Oaep::new::<Sha256>();

    let decrypted = priv_key.decrypt(padding_decrypt, &encrypted)?;
    println!("Расшифрованное сообщение: {}", String::from_utf8_lossy(&decrypted));

    Ok(())
}
