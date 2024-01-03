use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1).expect("no argument given");
    let mut file = File::open(&arg1).expect("file not found");

    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = file.read_exact(&mut buffer) {
        print!("{:08x}  ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("##"),
                _ => print!("{:02x} ", byte),
            }
        }

        print!("");
        pos += BYTES_PER_LINE;
    }
}
