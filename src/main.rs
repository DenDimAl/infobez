use std::io::stdin;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs8::{LineEnding, EncodePrivateKey, EncodePublicKey}, pkcs1v15::{SigningKey, Signature}, signature::Keypair};
use sha2::{Sha256, Digest};
use rsa::signature::{RandomizedSigner, Verifier};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let private_key_pem = priv_key.to_pkcs8_pem(LineEnding::LF)?;
    let public_key_pem = pub_key.to_public_key_pem(LineEnding::LF)?;
    
    let handle: std::io::Stdin = std::io::stdin();

    std::fs::write("private_key.pem", private_key_pem.as_bytes())?;
    std::fs::write("public_key.pem", public_key_pem.as_bytes())?;
    
    let mut message = String::from("");
    println!("Введите строку для шифрования");
    handle.read_line(&mut message);
    println!("\nИсходное сообщение: {}", message);
    
    let encrypted = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())?;
    
    println!("Зашифрованное сообщение: {:?}", encrypted);
    
    let decrypted = priv_key.decrypt(Pkcs1v15Encrypt, &encrypted)?;
    println!("Расшифрованное сообщение: {}", String::from_utf8_lossy(&decrypted));


    println!("Введите данные для подписи");
    
    let mut data = String::from("");
    handle.read_line(&mut data);
    println!("\nДанные для подписи: {}", data);
    
    let signing_key = SigningKey::<Sha256>::new_unprefixed(priv_key);
    let signature: Signature = signing_key.sign_with_rng(&mut rng, data.as_bytes());
    println!("Подпись: {}", signature);
    let mut corruption = String::from("");
    println!("Введите что-нибудь, чтобы испоганить данные, или оставьте всё так, как и было");
    handle.read_line(&mut corruption);
    if corruption.trim().len() > 0 {
        data = corruption.clone();
    }
    let verifying_key = signing_key.verifying_key();
    
    match verifying_key.verify(data.as_bytes(), &signature) {
        Ok(()) => println!(" Подпись верна!"),
        Err(e) => println!(" Ошибка проверки: {}", e),
    }
    
    Ok(())
}
