
use std::io::Result;

// 编译protobuf
fn main() -> Result<()> {
    // tonic_build::compile_protos("proto/records.proto")?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("proto")
        .compile(&["proto/records.proto"], &["proto"])?;

    Ok(())
}