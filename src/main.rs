use std::collections::HashMap;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn two_power(num: u32) -> u32{
    return 1 << num;
}

fn hide_a_secret(filename: &str, text: &mut String){

    let content: String = read_to_string(filename).unwrap();
    let a: Vec<char> = content.chars().collect();
    let mut secret: Vec<u32> = vec![];
    let mut ends: Vec<usize> = text.match_indices('\n').map(|(index, _)| index as usize).collect();
    for i in 0..a.len(){
        secret.push(a[i] as u32);
    }
    let mut ind: usize = 0; 
    for i in 0..secret.len(){
        for j in (0..32).rev() {
            if two_power(j) & secret[i] != 0{
                if ind < ends.len(){
                    text.insert(ends[ind] + 1, ' ');
                    if ind < ends.len() - 1{
                        for k in ind + 1..ends.len() - 1 {
                            ends[k] += 1;
                        }
                    }
                }
                else {
                    text.push_str("\n ");
                }
            } 
            else if ind >= ends.len(){
                text.push('\n');
            }
            ind+=1;
        }
    }
    
}

fn decipher(text: &String) -> String{
    let t:Vec<char> = text.chars().collect();
    let mut o: Vec<usize> = vec![];
    for i in 0..t.len(){
        if t[i] =='\n'{
            o.push(i);
        }
    }
    let mut u: u32 = 0;
    let mut pow: u32 = 32;
    let mut res: String = String::from("");
    for i in 0..o.len(){
        
        pow-=1;
        if o[i] == t.len() - 1 {
             res.push(char::from_u32(u).unwrap());
             break;
        }
        if t[o[i] + 1] == ' '{
            u |= two_power(pow);
        }
        
        if i % 32 == 0 || i == o.len() - 1 {
            res.push(char::from_u32(u).unwrap());
            u = 0;
        }
        if pow == 0 {
            pow = 32;
        }
    }

    return res;
}

fn cipher_secret(filename: &str, text: &mut String) {
    let content: String = read_to_string(filename).unwrap();
    let a: Vec<char> = content.chars().collect();
    let  mut b: Vec<char> = text.chars().collect();
    
    let mut secret: Vec<u32> = vec![];
    let mut analog_map = HashMap::new();
    
    analog_map.insert('а', 'a');
    analog_map.insert('е', 'e');
    analog_map.insert('о', 'o');
    analog_map.insert('р', 'p');
    analog_map.insert('с', 'c');
    analog_map.insert('у', 'y');
    analog_map.insert('х', 'x');
    analog_map.insert('к', 'k');
    analog_map.insert('м', 'm');
    analog_map.insert('т', 't');
    analog_map.insert('в', 'b');
    analog_map.insert('н', 'h');
    
    analog_map.insert('А', 'A');
    analog_map.insert('В', 'B');
    analog_map.insert('Е', 'E');
    analog_map.insert('К', 'K');
    analog_map.insert('М', 'M');
    analog_map.insert('Н', 'H');
    analog_map.insert('О', 'O');
    analog_map.insert('Р', 'P');
    analog_map.insert('С', 'C');
    analog_map.insert('Т', 'T');
    analog_map.insert('У', 'Y');
    analog_map.insert('Х', 'X');

    for i in 0..a.len(){
        secret.push(a[i] as u32);
    }

    let mut pow:u32 = 32;
    let mut inds:Vec<usize> = vec![];
    for i in 0..b.len() {
        if analog_map.get(&b[i]).is_some() {
            inds.push(i);
        }
        
    }
    let mut ind:usize = 0;
    
    for i in 0..secret.len(){
        for j in (0..32).rev() {
            if two_power(j) & secret[i] != 0 {
               b[inds[ind]] = *analog_map.get(&b[inds[ind]]).unwrap();
            }
            ind+=1;
        }
    }
    *text = b.into_iter().collect();
}

fn main() {
    let mut s:String = String::from("Давай будем честными, мы понятия не имеем, что тут можно написать. Мало того, 
    нужно написать что-то с определённым условием, мол, должно быть хотя бы 32 буквы-аналога. Это целая куча! Кто вообще в здравом уме
    будет писать осмысленный текст, причём аккуратненько, ручкой или пальчиком проводя по строке и проговаривая А, В, К,
    отлично ещё Энадцать штук таких же буковок! Безумие!");
    cipher_secret("C:/Users/ddeni/Downloads/lorem/lorem/hide_me.txt", &mut s);
    //s = s.replace(" ", "1");
    //s = s.replace("\n", "0");
    //println!("{}",s);
    //let res: String = decipher(&s);
   let a:char = 'г';
   println!("{}", a as u32);
    println!("{}",s);

}
