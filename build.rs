use anyhow::{Ok, Result};
use prost_build;
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Gravity", "abi/gravity.json")?
        .generate()?
        .write_to_file("src/abi/gravity.rs")?;

    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("./src/pb");
    prost_build.compile_protos(&["gravity.proto"], &["./proto/"])?;

    Ok(())
}
