use std::fs::File;
use std::io::prelude::*;
use std::env;

const BYTES_PER_LINE: usize = 16;

fn main(){ 
    let ad = "hello world";
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: process filename");
    let mut f = File::open(&fname).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    while let Ok(_) = f.read_exact(&mut buffer) {
        println!("[0x{:08}]", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!("."),
                0xff => print!("##"),
                _ => print!("{:02x}", byte)
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}