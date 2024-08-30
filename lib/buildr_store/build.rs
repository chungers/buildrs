fn main() {
    gen_proto("./src/store.proto", "./src/");
}

fn gen_proto(proto_file: &str, out_dir: &str) {
    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir)
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("!!! protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
