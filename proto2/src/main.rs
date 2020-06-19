use std::fs::File;
use std::io::{BufReader, Write};
extern crate clap;
use clap::{App, Arg};
extern crate chrono;
extern crate protobuf;
extern crate time;
use chrono::NaiveDate;
#[macro_use]
mod calc_age;
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
    let mut input_file_buffer = BufReader::new(input_file);
    let mut output_file = File::create(output_path).expect("unable to create output file");

    let mut stream = protobuf::CodedInputStream::from_buffered_reader(&mut input_file_buffer);

    while !stream.eof().expect("unable to read stream") {
        let person_from_file: Person = stream.read_message().expect("unable to read message");
        let person_dob = NaiveDate::parse_from_str(person_from_file.get_dob(), "%Y-%m-%d")
            .expect("unable to parse person date");
        let out = format!(
            "{:?} age:{}\n",
            person_from_file,
            calculate_age!(person_dob)
        );

        output_file
            .write_all(out.as_bytes())
            .expect("unable to write");
    }
}
