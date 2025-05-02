use std::path::Path;

fn main() {
    let proto_files = ["sonetto.proto", "cmd_id.proto"];

    for proto in &proto_files {
        println!("cargo::rerun-if-changed={proto}");
    }

    if proto_files.iter().all(|f| Path::new(f).exists()) {
        prost_build::Config::new()
            .out_dir("include/")
            .compile_protos(&proto_files, &["."])
            .unwrap();
    }
}
