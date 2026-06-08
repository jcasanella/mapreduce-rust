fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = ["registration.proto", "heartbeat.proto", "mapper.proto"];
    for proto in &protos {
        tonic_prost_build::compile_protos(proto)?;
    }
    Ok(())
}
