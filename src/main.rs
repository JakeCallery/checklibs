use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::Display;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::string::String;

extern crate argparse;
use argparse::{ArgumentParser, Store};

fn lines_from_file(file: &File) -> Vec<String> {
    let buff_reader = BufReader::new(file);
    buff_reader.lines()
       .map(|line| line.expect("Could not parse line"))
       .collect()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);

    let mut cust_lib_file_path: String = "".to_string();
    let mut req_lib_file_path: String = "".to_string();

    {//argparse scope
        let mut ap = ArgumentParser::new();
        ap.set_description("Compare a customer's rpm -qa output to list of requried libraries");
        ap.refer(&mut cust_lib_file_path).add_option(&["-c", "--custlibs"], Store, "Path to customer library list from rpm -qa").required();
        ap.refer(&mut req_lib_file_path).add_option(&["-r", "--reqlibs"], Store, "Path to required library list").required();
        //ap.parse_args_or_exit();
        match ap.parse_args() {
            Ok(()) => {},
            Err(err) => {
                println!("Argument Error: {:?}", err);
                std::process::exit(err);
            }
        }
    }//argparse scope

    //let cust_libs_path: &Path = Path::new("./src/rpmoutput.txt");
    let cust_libs_path: &Path = Path::new(&cust_lib_file_path);
    let cust_libs_display: Display = cust_libs_path.display();

    //let req_libs_path: &Path = Path::new("./src/reqlibs_19.1.txt");
    let req_libs_path: &Path = Path::new(&req_lib_file_path);
    let req_libs_display: Display = req_libs_path.display();

    let cur_path: PathBuf = env::current_dir().unwrap();
    let cur_path_disp: Display = cur_path.display();
    println!("CWD: {}", cur_path_disp);

    println!("Opening Customer File");
    let cust_libs_file: File = match File::open(&cust_libs_path) {
        Err(why) => panic!("Couldn't read {}: {}", cust_libs_display, why.description()),
        Ok(cust_libs_file) => cust_libs_file,
    };

    println!("Opening Lib List");
    let req_libs_file: File = match File::open(&req_libs_path) {
        Err(why) => panic!("Couldn't Read {}: {}", req_libs_display, why.description()),
        Ok(req_libs_file) => req_libs_file,
    };

    println!("Saving Customer lines to memory");
    #[allow(unused_variables)]
    let cust_lines_itr: Vec<String> = lines_from_file(&cust_libs_file);

    println!("Saving Required lines to memory");
    #[allow(unused_variables)]
    let req_lines_itr: Vec<String> = lines_from_file(&req_libs_file);

    for line in cust_lines_itr {
        println!("{:?}", line);
    }

    for line in req_lines_itr {
        println!("{:?}", line);
    }

}
