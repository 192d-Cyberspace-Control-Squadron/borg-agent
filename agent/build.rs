fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false) // agent is a gRPC client only
        .compile_protos(&["proto/agent.proto"], &["proto"])?;
    Ok(())
}
