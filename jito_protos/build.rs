use tonic_build::configure;

fn main() {
    println!("cargo:rerun-if-changed=protos/");
    configure()
        .protoc_arg("--experimental_allow_proto3_optional") // Add this line
        .compile(
            &[
                "protos/auth.proto",
                "protos/block.proto",
                "protos/block_engine.proto",
                "protos/bundle.proto",
                "protos/packet.proto",
                "protos/relayer.proto",
                "protos/searcher.proto",
                "protos/shared.proto",
            ],
            &["protos"],
        )
        .expect("Failed to compile protos");
}
