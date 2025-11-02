fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure output directory exists
    std::fs::create_dir_all("src/generated")?;
    
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile(&["proto/match.proto"], &["proto"])?;
    
    // Tell cargo to rerun if proto changes
    println!("cargo:rerun-if-changed=proto/match.proto");
    
    Ok(())
}

