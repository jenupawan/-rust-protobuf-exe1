extern crate clap;
extern crate protobuf;
use clap::{App, Arg};
use protobuf::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};

mod person;
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
    let input_buffer = BufReader::new(input_file);
    let mut ofile = File::create(output_path).expect("unable to create file");

    for line in input_buffer.lines() {
        let each_line = line.unwrap();
        let mut values: Vec<String> = Vec::new();
        for i in each_line.split(',') {
            values.push(i.to_string())
        }

        let mut person: person_obj = person_obj::new();
        person.set_last_name(values[0].to_owned());
        person.set_first_name(values[1].to_owned());
        person.set_dob(values[2].to_owned());

        let byte_info = person.write_length_delimited_to_bytes().unwrap();
        ofile.write_all(&byte_info).expect("unable to write");
    }
}
