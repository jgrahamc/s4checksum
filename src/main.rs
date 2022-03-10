// s4checksum - calculate checksum of binary file in same format
// as the Dataman S4 programmer
//
// Binary file on stdin, output the checksum on stdout
//
// Copyright (c) 2022 John Graham-Cumming

use std::io;
use std::io::Read;

fn main() {
   let mut checksum: u32 = 0;
   
   for b in io::stdin().bytes() {
       let c = b.unwrap() as u32;
       checksum += c;
    }

    println!("{:08X}", checksum);
}
