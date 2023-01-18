fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_transport(true)
        .build_client(true)
        // .out_dir("src/generated") // you can change the generated code's location
        .compile(
            &["proto/massa.proto"],
            &["proto/"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
