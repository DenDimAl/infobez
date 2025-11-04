use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, BufWriter, Write};

use sha2::{Sha256, Digest};

fn is_only_latin_alphanumeric_bytes(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_alphanumeric())
}

fn register_user(login:&str, password:&str, filename:&str) {
    if !is_only_latin_alphanumeric_bytes(login) || !is_only_latin_alphanumeric_bytes(password){
        println!("В пароле или логине могут быть только буквы латинского алфавита и цифры");
        return;
    }
    let mut file = OpenOptions::new().append(true).create(true).open(filename).unwrap();
    let content = read_to_string(filename).unwrap();
    if !content.find(login).is_none() {
        return;
    }
    let mut hasher = Sha256::new();
    hasher.update(password);
    let res = hasher.finalize();
    let hash_string = format!("{:x}", res);
    file.write_all(login.as_bytes());
    file.write_all(b":");
    file.write_all(hash_string.as_bytes());
    file.write_all(b"\n");
}

fn login_user(login: &str, password:&str, filepath: &str) -> bool {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    for (line_number, line_result) in reader.lines().enumerate(){
        let line = line_result.unwrap();
        let name = &line[0..line.find(":").unwrap()];
        if name != login{
           continue;
        }
        
        let h =  &line[line.find(":").unwrap() + 1..line.len()];
        let mut hasher = Sha256::new();
        hasher.update(password);
        let res = hasher.finalize();
        let res_string = format!("{:x}",  res);
       // println!("{}, {}", h, res_string);
        if res_string == h {
            println!("Добрый день, {}, Чем могу помочь?", login);
            return true;
        }
    }
   return false;
}

fn attack(login:&str, filepath: &str){
    let symbols: [char; 62] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    let mut answer  = vec![symbols[0]];
    while !login_user(login, &answer.iter().collect::<String>(), filepath){
        next(&mut answer,&symbols);
    }
}



fn next(a: &mut Vec<char>, symb: &[char; 62]) -> bool {
    for i in (0..a.len()).rev() {
        let cur = symb.iter().position(|&x| x == a[i]).unwrap();
        if cur < 61 {
            a[i] = symb[cur + 1];
            return true;
        } else {
            a[i] = symb[0];
        }
    }
    // Если все разряды переполнены, добавляем новый символ
    a.insert(0, symb[0]);
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    register_user("login", "sosal", "secret.txt");
    register_user("hey", "da", "secret.txt");
    login_user("login", "sosal", "secret.txt");
    let mut a: i32;
    let handle: std::io::Stdin = std::io::stdin();
    let mut smth = String::new();
    let mut l = String::new();
    let mut p = String::new();
    let mut user = String::new();

    loop {
         
        if user.is_empty(){
            println!("Добро пожаловать, для начала работы, пожалуйста, авторизируйтесь");
            handle.read_line(&mut l)?;
            handle.read_line(&mut p)?;
           if login_user(l.trim(), p.trim(), "secret.txt"){
                user = l.clone();
                
           }
           else{
                 println!("Неправильно введён логин или пароль, повторите попытку");
                 continue;
           }
        }
        l.clear(); 
        p.clear();
        println!("Список возможных команд:");
        println!("1: Зарегистрировать нового пользователя;");
        println!("2: Сделать что-то;");
        println!("3: Сменить пользователя;");
        println!("4: Завершить работу");
        
        handle.read_line(&mut smth)?;
        println!("{}", smth);
        a = smth.trim().parse::<i32>()?;
        smth.clear();
        match a {
            1 => {
                println!("Введите логин и пароль для нового пользователя");
                handle.read_line(&mut l)?;
                handle.read_line(&mut p)?;
                register_user(l.trim(), p.trim(), "secret.txt");
            }
            2 => {
                println!("Ну, что определённо произошло...");
            }
            3 => {
                user.clear();
            }
            4 => {
                println!("До свидания");
                break;  
            } 
            _ => {
                println!("Неверно введена команда, повторите попытку");
            }
        }

    }
    Ok(())
}
