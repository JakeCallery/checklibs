use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::Display;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::string::String;

fn lines_from_file(file: &File) -> Vec<String> {
    let buff_reader = BufReader::new(file);
    buff_reader.lines()
       .map(|line| line.expect("Could not parse line"))
       .collect()
}

fn main() {

    let cust_libs_path: &Path = Path::new("./src/rpmoutput.txt");
    let cust_libs_display: Display = cust_libs_path.display();

    let req_libs_path: &Path = Path::new("./src/reqlibs_19.1.txt");
    let req_libs_display: Display = req_libs_path.display();

    let cur_path: PathBuf = env::current_dir().unwrap();
    let cur_path_disp: Display = cur_path.display();
    println!("CWD: {}", cur_path_disp);

    println!("Opening File");
    let file: File = match File::open(&cust_libs_path) {
        Err(why) => panic!("Couldn't read {}: {}", cust_libs_display, why.description()),
        Ok(file) => file,
    };

    println!("Saving lines to memory");
    let lines_itr: Vec<String> = lines_from_file(&file);

/*
    for line in lines_itr {
        println!("{:?}", line);
    }
*/

}
