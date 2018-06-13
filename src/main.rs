use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::Display;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;

fn main() {

    let path: &Path = Path::new("./src/rpmoutput.txt");
    let display: Display = path.display();

    let cur_path: PathBuf = env::current_dir().unwrap();
    let cur_path_disp: Display = cur_path.display();
    println!("CWD: {}", cur_path_disp);

    println!("Opening File");
    let mut file: File = match File::open(&path) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut str: String = String::new();
    match file.read_to_string(&mut str) {
        Err(why) => panic!("Could not read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:{}\n", display, str),
    }


}
