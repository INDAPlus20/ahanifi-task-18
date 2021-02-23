use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main(){
    create_index_file("../token.txt".to_string());
}

fn create_magic_file(path:String){
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file));

    
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
        let line=l.unwrap().clone();
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
