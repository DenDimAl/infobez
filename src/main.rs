
use aes::Aes256;
use rand::RngCore;
use magic_crypt::{MagicCrypt256, MagicCryptTrait, new_magic_crypt};
use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;
use base64::{encode, decode};



fn generate_aes256_key() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    return base64::encode(key);
}

fn generate_iv() -> String {
    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut iv);
    return base64::encode(iv);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle: std::io::Stdin = std::io::stdin();
    let mut crypt = String::new();
    println!("Введите путь до файла для сохранения шифротекста");
    handle.read_line(&mut crypt)?;
    let mut file_c = OpenOptions::new().create(true).read(true).write(true).open(crypt.trim()).unwrap();
    let mut keys = String::new();
    println!("Введите путь до файла для сохранения ключей");
    handle.read_line(&mut keys)?;
    let mut file_k = OpenOptions::new().create(true).read(true).write(true).open(keys.trim()).unwrap();
    
    println!("Введите текст, который нужно зашифровать");
    let mut message = String::from("");
    handle.read_line(&mut message);

    let key_bytes = generate_aes256_key();
    let iv = generate_iv();

    file_k.write_all(b"key:");
    file_k.write_all(key_bytes.as_bytes());
    file_k.write_all(b"\n");
    file_k.write_all(b"iv:");
    file_k.write_all(iv.as_bytes());
        
    let mc = MagicCrypt256::new(key_bytes, Some(iv));

    let cipher_text = mc.encrypt_str_to_base64(message);

    file_c.write_all(b"ciphered text:");
    file_c.write_all(cipher_text.as_bytes());

    

    println!("Шифротекст, ключ и iv были сохранены в файлы. Введите что-нибудь, чтобы дешифровать:");
    let mut placeholder = String::from("");
    handle.read_line(&mut placeholder);

    let mut file_c = OpenOptions::new().create(true).read(true).write(true).open(crypt.trim()).unwrap();
    let mut file_k = OpenOptions::new().create(true).read(true).write(true).open(keys.trim()).unwrap();
    
    
    let mut encrypted_message =String::from("");
    file_c.read_to_string(&mut encrypted_message)?;
    encrypted_message = String::from_str(&encrypted_message[encrypted_message.find(":").unwrap()+1..encrypted_message.len()])?;
    let mut keys_content = String::from("");
    file_k.read_to_string(&mut keys_content)?;
    let mut iv_f = String::from_str(&keys_content[keys_content.find("iv:").unwrap()+3..keys_content.len()])?;
    let mut key_f = String::from_str(&keys_content[keys_content.find("key:").unwrap()+4..keys_content.find("\n").unwrap()])?;
    
    println!("{}", encrypted_message);
    println!("{}", key_f);
    println!("{}", iv_f);

    let new_mc = MagicCrypt256::new(key_f, Some(&iv_f));

    match new_mc.decrypt_base64_to_string(&encrypted_message){
         Ok(decrypted) => {
            println!("Расшифрованный текст: {}", decrypted);
            
        }
        Err(e) => println!("Ошибка дешифрования"),
    }

    

    Ok(())

}