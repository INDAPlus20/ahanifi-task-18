use core::panic;
use std::{cmp::min, env, fs::File};
use std::io::{BufRead, BufReader, Read};
use std::{error::Error, io::Write, str::Chars, usize};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use env::args;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1){
        Some(s) =>{
            match s.trim() {
                "index" => create_index_file("../token.txt".to_string()),
                "magic" => create_magic_file("../index_file.txt".to_string()),
                _ =>{},
            }
        }
        None => {}
    }
}

fn create_magic_file(path: String) {
    let mut file = File::open(path).unwrap();
    // let mut reader = BufReader::new(
    //     DecodeReaderBytesBuilder::new()
    //         .encoding(Some(WINDOWS_1252))
    //         .build(file),
    // );
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);

    const permutations: usize = 30 * 30 * 30;
    let mut indicies = [-1; permutations];

    let mut word_length=0usize;
    let mut current_word:Vec<u8>=vec![];
    let mut is_word = true;
    
    let mut prefix:Vec<u8>=vec![];

    for (i, b) in buf.iter().enumerate() {
        match b {
            b' ' => {
                if is_word && prefix != current_word {
                    let index = calculate_index_from_byte(&prefix);
                    indicies[index-1] = (i  - word_length) as isize;
                    
                    // println!("{}", std::str::from_utf8(&prefix).unwrap());
                    // println!("{}",index);
                    // panic!();
                }
                is_word = false;
            }
            b'\n' => {
                word_length=0;
                current_word=prefix.clone();
                prefix.clear();
                is_word = true;
            }
            byte => {
                word_length+=1;
                if is_word && prefix.len()<3{
                    prefix.push(*byte);
                }
            }
        }
    }

    let mut f = File::create("../magic_johnson.txt").expect("Unable to create file");
    for i in &indicies {
        f.write_all((i).to_string().as_bytes())
            .expect("Unable to write data");
        f.write_all(&[b'\n']);
    }
    f.flush();

    //let index= a as usize -97 + (b as usize -97)*30 + (c as usize -97)*30*30;
}

fn calculate_index_from_byte(bytes: &Vec<u8>) -> usize {
    let mut index = 0;
    let mut iter = bytes.iter();
    for i in (0..3).rev() {
        let char_val = match iter.next() {
            Some(228) => {28},
            Some(229) => {27},
            Some(246) => {29},
            Some(n) => n - 96,
            None => 0,
        };
        index += 30usize.pow(i) * char_val as usize
    }
    index
}

fn calculate_index(chars: &mut Chars) -> usize {
    let mut index = 0;
    for i in (0..3).rev() {
        let char_val = match chars.next().unwrap() {
            'ö' => 29,
            'ä' => 28,
            'å' => 27,
            n => n as usize - 96,
        };
        index += 30usize.pow(i) * char_val
    }
    index
}

fn create_index_file(path: String) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    );

    let mut working_string = String::new();
    let mut current_index = String::new();

    for l in reader.lines() {
        let line = l.unwrap();
        if line.len() == 0 {
            break;
        }

        let mut split_line = line.trim().split_ascii_whitespace();
        let index = split_line.next().unwrap();

        if index != current_index {
            working_string.push_str("\n");
            working_string.push_str(index);
            current_index = index.to_string();
        }
        for value in split_line {
            working_string.push(' ');
            working_string.push_str(value);
        }
    }
    working_string.remove(0);
    let encoded = WINDOWS_1252.encode(&working_string).0;
    std::fs::write("../index_file.txt", encoded);
}

// format!("{:0>3b}", index_registry);
