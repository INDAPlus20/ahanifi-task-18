use std::{cmp::{max, min}, env, fs, io::{Read, Seek, SeekFrom}, str::{Chars, from_utf8}, usize};
use encoding_rs::WINDOWS_1252;
use fs::{File, read};

fn main() {
    let index_path="../index_file.txt";
    let magic_path="../magic_johnson.txt";

    let mut index_file=File::open(index_path).unwrap();

    let magic_file=fs::read_to_string(magic_path).unwrap();
    let magic_lines:Vec<&str>=magic_file.split("\n").collect();

    let mut korpus_file=File::open("../korpus").unwrap();
    let korpus_size=korpus_file.metadata().unwrap().len();

    let margin=40;

    let args: Vec<String> = env::args().collect();
    let input;
    if let Some(s)= args.get(1){
        input=s;
    }else{
        println!("Please specify which word you want to look up");
        return;
    }

    let magic_index=calculate_index(&mut input.chars());
    let i=magic_lines[magic_index-1].parse::<isize>().unwrap();

    if i == -1{
        println!("Word doesn't exist");
        return;
    }
    
    let mut cap;
    if magic_index+1 != magic_lines.len(){
        cap=korpus_size;
        for j in magic_index+1..magic_lines.len(){
            let val=magic_lines[j].parse::<isize>().unwrap();
            if val != -1{
                cap=val as u64;
                break;
            }
        }
    }else{
        cap=korpus_size;
    }

    index_file.seek(SeekFrom::Start(i as u64)).unwrap();

    let mut buf=Vec::<u8>::new();
    index_file.take(cap-i as u64).read_to_end(&mut buf).unwrap();
    let decoded=WINDOWS_1252.decode(&buf).0;
    let string=decoded.split("\n");

    for line in string{
        let mut split_line=line.split_whitespace();
        let next_word=split_line.next();
        if next_word == Some(input){
            let occurences:Vec<&str>=split_line.collect();
            for occ in occurences.iter(){

                let occ_int=occ.parse::<u64>().unwrap();
                let start_pos=max(occ_int-margin,0);
                let end_pos=min(occ_int+margin,korpus_size);

                let reference=korpus_file.by_ref();
                reference.seek(SeekFrom::Start(start_pos)).unwrap();
                

                let chunksize=end_pos-start_pos;

                let mut buf=Vec::<u8>::with_capacity(chunksize as usize);
                reference.take(chunksize).read_to_end(&mut buf);
                let decoded=WINDOWS_1252.decode(&buf).0.trim().replace("\n", "|");
               
                println!("... {} ...",decoded);
                println!("----------------------------------");
                
            }
            break;
        }
    }

    println!("Hello, world!");
}

    fn calculate_index(chars: &mut Chars) -> usize {
        let mut index = 0;
        for i in (0..3).rev() {
            let char_val = match chars.next() {
                Some('ö') => 29,
                Some('ä') => 28,
                Some('å') => 27,
                Some(n) => n as usize - 96,
                None => 0,
            };
            index += 30usize.pow(i) * char_val
        }
        index
    }
    
