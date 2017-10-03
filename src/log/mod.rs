use std::fs::File;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;

pub fn log(text: String) {
    let path = Path::new("/tmp/debug.txt");
    let display = path.display();
    let mut file = match OpenOptions::new().append(true).open(&path) {
        Err(why) => panic!("unable to open file {} reason:{}", display, why.description()),
        Ok(file) => file,
    };

    write!(file, "{}", text).expect("unable to write to file");
}
