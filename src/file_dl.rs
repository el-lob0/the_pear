use std::process::Command;

pub fn download_image(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "./image_store/image.gif";

    // Run curl
    let status = Command::new("curl")
        .args([
            "-L",  // follow redirects
            "-o", output_path,
            url,
        ])
        .status()?; // .status() waits for exit code, .output() captures stdout/stderr

    if status.success() {
        println!("✅ Downloaded successfully to {}", output_path);
    } else {
        eprintln!("❌ curl failed with exit code {:?}", status.code());
    }

    Ok(())
}
