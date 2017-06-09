extern crate phash;
// use phash::Cityhash;
// use phash::Hash16;
use phash::Hash16;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

    // let string: &str = "abcdefg";
    // let val = phash::Cityhash::to_16bit(string);
    // println!("{} ==> {:x}", string, val);

    let file = match File::open("test.txt") {
        Err(why) => panic!("couldn't open !! {}", why),
        Ok(file) => file,
    };

    let file = BufReader::new(&file);
    for line in file.lines() {
        let s = match line {
            Err(why) => panic!("couldn't open !! {}", why),
            Ok(s) => s,
        };
        // println!("| {} \t|", s);
        println!("| {} \t| {:x} \t|", s, phash::Cityhash::to_16bit(&s));
    }
}
