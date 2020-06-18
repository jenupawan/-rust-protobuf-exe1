extern crate clap;
extern crate protobuf;
use clap::{App, Arg};
use protobuf::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};

mod person;
use person::Person;
fn main() {
    let matches = App::new("proto")
        .version("1.0")
        .about("example of using protofobuff")
        .author("Pawan Jenu")
        .arg(
            Arg::with_name("input")
                .short("-i")
                .long("-input_file")
                .help("path for input file")
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

    let input_path = matches
        .value_of("input")
        .expect("invalid input path argument");
    let output_path = matches
        .value_of("output")
        .expect("invalid output path argument");
    let input_file = File::open(input_path).expect("input file not found");
    let input_file_buffer = BufReader::new(input_file);
    let mut output_file = File::create(output_path).expect("unable to create output file");

    for line in input_file_buffer.lines() {
        let each_line = line.expect("unable to read line");
        let line_info: Vec<&str> = each_line.split(',').collect();
        let mut person: Person = Person::new();

        person.set_last_name(line_info[0].to_owned());
        person.set_first_name(line_info[1].to_owned());
        person.set_dob(line_info[2].to_owned());

        let person_byte_info = person
            .write_length_delimited_to_bytes()
            .expect("unable convert into bytes");
        output_file
            .write_all(&person_byte_info)
            .expect("unable to write to output file");
    }
}
