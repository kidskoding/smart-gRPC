use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("sentinel_descriptor.bin"))
        .compile_protos(
            &["proto/sentinel.proto"],
            &["proto"],
        )?;

    Ok(())
}