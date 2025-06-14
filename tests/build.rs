fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .enum_attribute("ActivityType", "#[derive(serde::Deserialize)]")
        .compile_protos(&["proto/posts.proto", "proto/stats.proto"], &["proto"]).unwrap();
    Ok(())
}
