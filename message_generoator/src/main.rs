extern crate protoc_rust;
fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos/")
        .inputs(&["proto_file/person.proto"])
        .include("proto_file")
        .run()
        .expect("unable to generate rust file from the proto file");
}
