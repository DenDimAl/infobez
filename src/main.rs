use std::io::Read;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

fn find_files_in_dir(dir: &std::path::PathBuf) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files:Vec<PathBuf> = Vec::new();
    
    for entry in fs::read_dir(dir)?{
        let cur_entry = entry?;
        let cur_path = cur_entry.path();
        if cur_path.is_dir(){
            files.extend(find_files_in_dir(&cur_path)?);
        } else {
            files.push(cur_path);
        }
    }
    return  Ok(files);
}

fn signature(f: &std::path::PathBuf, s: usize) -> std::io::Result<Vec<u8>>{
    let mut data = File::open(f)?; // Vec<u8>
    let mut buf  = vec![0u8; s];
    data.read_exact(&mut buf)?;
    return Ok(buf);
}

fn main() -> std::io::Result<()> {
    let mut f: String = String::from("");

    let handle: std::io::Stdin = std::io::stdin();

    let mut s: usize = 24;

    println!("Введите путь до файла с нужной сигнатурой");

    handle.read_line(&mut f)?;
    println!("Файл меньше указанной длины сигнатуры, размер был уменьшен соответственно");
    
    let buf = signature(&std::path::PathBuf::from(f.trim()), s)?;
    
    println!("Сигнатура: {:?}", buf.clone());
    
    println!("Введите путь до папки с искомыми файлами");
    let mut dir:String = String::from("");
    handle.read_line(&mut dir)?;
    
    let files = find_files_in_dir(&std::path::PathBuf::from(dir.trim()))?;

    for f in files{
        let sig = signature(&f, s)?;
        if  sig == buf.clone(){
            println!("Папка: {}, Сигнатура: {:?}", f.to_string_lossy(), sig);
        } 
    }

    Ok(())
}