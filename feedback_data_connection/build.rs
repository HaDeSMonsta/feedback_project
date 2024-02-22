use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    tonic_build::compile_protos("proto/comm.proto")?;
    Ok(())
}