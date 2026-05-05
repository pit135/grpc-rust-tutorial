fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/services.proto"], // Pastikan path ke file .proto sudah benar
            &["proto"],                // Folder tempat file .proto berada
        )?;
    Ok(())
}