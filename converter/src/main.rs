use std::{error::Error, io::Write, str::Chars, usize};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main(){
    create_magic_file("index_file.txt".to_string());
    //create_index_file("../token.txt".to_string());
}

fn create_magic_file(path:String){
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file));

    const permutations:usize=30*30*30;
    let mut indicies=[-1;permutations];
    for (i,l )in reader.lines().enumerate(){
        let line = l.unwrap();
        if line.len()==0{
            continue;
        }
        let mut words=line.split_whitespace();
        let index_string=format!("{:`<3}",words.next().unwrap());
        let mut chars=index_string.chars();
        let index=calculate_index(&mut chars);
        indicies[index]=i as isize;
    }
    let mut f = File::create("magic_johnson.txt").expect("Unable to create file");                                                                                                          
    for i in &indicies{
        f.write_all(&[b'\n']);                                                                                                                                                                  
        f.write_all((i).to_string().as_bytes()).expect("Unable to write data");                                                                                                                            
    }  

    //let index= a as usize -97 + (b as usize -97)*30 + (c as usize -97)*30*30;
}

fn calculate_index(chars:&mut Chars) -> usize{
    let mut index=0;
    for i in (0..3).rev(){
        let char_val=match chars.next().unwrap() {
            'ö' => 29,
            'ä' => 28,
            'å' => 27,
            n=> n as usize -96,

        };
        index+=30usize.pow(i)*char_val
    }
    index
}

fn create_index_file(path:String){

    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file));

    let mut working_string=String::new();
    let mut current_index=String::new();
    
    for l in reader.lines() {
        let line=l.unwrap();
        if line.len()==0{
            break;
        }

        let mut split_line=line.trim().split_ascii_whitespace();
        let index= split_line.next().unwrap();

        if index != current_index{
            working_string.push_str("\n");
            working_string.push_str(index);
            current_index=index.to_string();
        }
        for value in split_line{
            working_string.push(' ');
            working_string.push_str(value);
            
        }
    }
    let encoded =WINDOWS_1252.encode(&working_string).0;
    std::fs::write("index_file.txt", encoded);
}

// format!("{:0>3b}", index_registry);
