fn main() {
    sonetto_gen("sonetto.proto");
}

fn sonetto_gen(proto_file: &str) {
    if std::path::Path::new(proto_file).exists() {
        println!("cargo::rerun-if-changed={proto_file}");
        prost_build::Config::new()
            .out_dir("include/")
            .compile_protos(&[proto_file], &["."])
            .unwrap();
    }
}
