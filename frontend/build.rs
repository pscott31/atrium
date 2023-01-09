use std::io;

fn main() -> io::Result<()> {
    // tonic_build::compile_protos("proto/atrium.proto")?;

    // tonic_build::configure()
    //     .build_server(false)
    //     .build_client(true)
    //     .compile(&["proto/atrium.proto"], &["proto"])

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/proto")
        .compile(&["proto/atrium.proto"], &["proto"])
}
