use std::collections::HashMap;
use std::collections::BTreeSet;
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

fn decipher_secret(text: &String) -> String{
    let t:Vec<char> = text.chars().collect();
    let mut analog_set_back = BTreeSet::new();
    
    analog_set_back.insert('a');
    analog_set_back.insert('e');
    analog_set_back.insert('o');
    analog_set_back.insert('p');
    analog_set_back.insert('c');
    analog_set_back.insert('y');
    analog_set_back.insert('x');
    analog_set_back.insert('k');
    analog_set_back.insert('m');
    analog_set_back.insert('t');
    analog_set_back.insert('b');
    analog_set_back.insert('h');
    
    analog_set_back.insert('A');
    analog_set_back.insert('B');
    analog_set_back.insert('E');
    analog_set_back.insert('K');
    analog_set_back.insert('M');
    analog_set_back.insert('H');
    analog_set_back.insert('O');
    analog_set_back.insert('P');
    analog_set_back.insert('C');
    analog_set_back.insert('T');
    analog_set_back.insert('Y');
    analog_set_back.insert('X');

    let mut analog_set = BTreeSet::new();
    
    analog_set.insert('а');
    analog_set.insert('е');
    analog_set.insert('о');
    analog_set.insert('р');
    analog_set.insert('с');
    analog_set.insert('у');
    analog_set.insert('х');
    analog_set.insert('к');
    analog_set.insert('м');
    analog_set.insert('т');
    analog_set.insert('в');
    analog_set.insert('н');
    
    analog_set.insert('А');
    analog_set.insert('В');
    analog_set.insert('Е');
    analog_set.insert('К');
    analog_set.insert('М');
    analog_set.insert('Н');
    analog_set.insert('О');
    analog_set.insert('Р');
    analog_set.insert('С');
    analog_set.insert('Т');
    analog_set.insert('У');
    analog_set.insert('Х');

    let mut u: u32 = 0;
    let mut pow: u32=32;
    let mut answer:String = String::from("");

    for i in 0..t.len(){
        if analog_set.get(&t[i]).is_some() {
            pow -=1;
        }
        else if analog_set_back.get(&t[i]).is_some(){
            u |= two_power(pow - 1);
            pow -=1;
           // println!("умножилось из-за {}, pow: {}, u: {}", t[i], pow, u);
        }
        if pow == 0 {

            pow = 32;
            answer.push(char::from_u32(u).unwrap());
            u = 0;
        }
    }

    return answer;
}

fn main() {
    
    let mut s:String = String::from("Давай будем честными, мы понятия не имеем, что тут можно написать. Мало того, 
    нужно написать что-то с определённым условием, мол, должно быть хотя бы 32 буквы-аналога. Это целая куча! Кто вообще в здравом уме
    будет писать осмысленный текст, причём аккуратненько, ручкой или пальчиком проводя по строке и проговаривая А, В, К,
    отлично ещё Энадцать штук таких же буковок! Безумие!");
    let mut smth:String = String::from("Посмотрите\nМаяковским я\nбудто стал\nсижу, пишу\nИ вообще\nрифмоплёт");
    hide_a_secret("C:/Users/ddeni/Downloads/lorem/lorem/hide_me.txt", &mut smth);
    cipher_secret("C:/Users/ddeni/Downloads/lorem/lorem/hide_me.txt", &mut s);
    let mut n:Vec<char> = s.chars().collect();
    //println!("{}", s);
    let res0:String = decipher(&smth);
    let res:String = decipher_secret(&s);
    println!("{}", res0);
    println!("{}", res);

}
