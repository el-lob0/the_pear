use std::process::Command;




pub fn call_gemini(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = dotenv::var("AI_API_KEY")?;

    // println!("{prompt}");
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

    let data = format!(
        r#"{{
            "contents": [
                {{
                    "parts": [
                        {{ "text": "{}" }}
                    ]
                }}
            ]
        }}"#,
        prompt
    );

    let output = Command::new("curl")
        .args([
            "-s",
            "-X", "POST",
            "-H", &format!("x-goog-api-key: {}", api_key),
            "-H", "Content-Type: application/json",
            "-d", &data,
            url,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}
