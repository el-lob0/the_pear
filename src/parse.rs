use regex::Regex;

/// Extracts the first `"text": "..."` value from a Gemini JSON string.
///
/// Returns `Some(String)` if found, or `None` otherwise.



pub fn extract_response(json_str: &str) -> String {
    // Regex to match: "text": "SOME TEXT"
    let re = Regex::new(r#"(?s)"text"\s*:\s*"((?:\\.|[^"\\])*)""#).unwrap();

    if let Some(caps) = re.captures(json_str) {
        let mut reply = caps
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "No text field found.".to_string());

        // Replace escaped newline sequences with actual newlines
        while reply.contains("\\n") {
            reply = reply.replace("\\n", "\n");
        }

        // Optionally unescape other sequences if needed
        reply = reply.replace("\\\"", "\"").replace("\\\\", "\\");

        reply
    } else {
        "No text field found.".to_string()
    }
}


