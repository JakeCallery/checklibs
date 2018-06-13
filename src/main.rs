use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::Display;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::string::String;

/*
fn lines_from_file(file: &File) -> Vec<String> {
    let buff_reader = BufReader::new(file);
    buff_reader.lines()
       .map(|line| line.expect("Could not parse line"))
       .collect()
}
*/

fn main() {

    let path: &Path = Path::new("./src/rpmoutput.txt");
    let display: Display = path.display();

    let cur_path: PathBuf = env::current_dir().unwrap();
    let cur_path_disp: Display = cur_path.display();
    println!("CWD: {}", cur_path_disp);

    println!("Opening File");
    let file: File = match File::open(&path) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(file) => file,
    };

    //let lines: Vec<String> = lines_from_file(&file);

    let buf_reader: BufReader<&File> = BufReader::new(&file);
    let lines_itr = buf_reader.lines();
    let mut lines: Vec<String> = Vec::new();

    for line in lines_itr {
        //println!("{:?}", line);
        match line {
            Err(why) => panic!("Could not parse line: {}", why.description()),
            Ok(_) =>
                {
                    let str:String = line.unwrap();
                    println!("{:?}", str);
                    lines.push(str);
                },
        }
    }

}
