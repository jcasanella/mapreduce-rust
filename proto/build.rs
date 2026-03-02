fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = ["registration.proto", "heartbeat.proto"];
    for proto in &protos {
        tonic_prost_build::compile_protos(proto)?;
    }
    Ok(())
}
