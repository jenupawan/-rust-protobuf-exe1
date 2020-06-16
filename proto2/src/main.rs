use std::fs::File;
use std::io::{BufReader, Write};
extern crate clap;
use clap::{App, Arg};
extern crate protobuf;
extern crate chrono;
extern crate time;
use chrono::{ Utc,NaiveDate};
use chrono::offset::{TimeZone};
mod person;
mod calc_age;
use calc_age::calculate_age as calculate_age;
use person::Person as person_obj;
fn main() {
    let matches = App::new("proto")
    .version("1.0")
    .about("example of using protofobuff")
    .author("Pawan Jenu")
    .arg(
        Arg::with_name("input")
            .short("-i")
            .long("-input_file")
            .help("path for input file with data")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("output")
            .short("-o")
            .long("-output_file")
            .help("path for output file")
            .takes_value(true),
    )
    .get_matches();

let input_path = matches.value_of("input").unwrap_or("data/input.txt");
let output_path = matches.value_of("output").unwrap_or("data/output.txt");
let input_file = File::open(input_path).expect("file not found");
let mut ofile = File::create(output_path).expect("unable to create file");
let mut input_buffer = BufReader::new(input_file);
    
let mut stream = protobuf::CodedInputStream::from_buffered_reader(&mut input_buffer);

    while !stream.eof().unwrap_or(false) {

        let person_from_file:person_obj = stream.read_message().unwrap();
        // println!("{:?}",person_from_file );
        let  date_only = NaiveDate::parse_from_str("2020-01-15", "%Y-%m-%d");
        let  person_date = Utc.from_local_date(&date_only.unwrap()).unwrap();
        let out = format!("{:?} age:{}\n",person_from_file,calculate_age(person_date));
        ofile.write_all(out.as_bytes()).expect("unable to write");
    }

}
