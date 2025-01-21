use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Nameservers {
    nameservers: Vec<String>,
}

fn generate_html(nameservers: &Vec<String>) -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n");
    html_content.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html_content.push_str("    <title>Nameservers</title>\n</head>\n<body>\n");
    html_content.push_str("    <h1>Nameservers</h1>\n    <ul>\n");

    for nameserver in nameservers {
        html_content.push_str(&format!("        <li>{}</li>\n", nameserver));
    }

    html_content.push_str("    </ul>\n</body>\n</html>");
    html_content
}

fn save_to_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    // Example input (could come from a file, another tool, or command-line args)
    let json_input = r#"{
        "nameservers": [
            "ns1.example.com",
            "ns2.example.com",
            "ns3.example.com"
        ]
    }"#;

    // Deserialize JSON input
    let nameservers: Nameservers = serde_json::from_str(json_input)?;

    // Generate HTML content
    let html_content = generate_html(&nameservers.nameservers);

    // Save to HTML file
    match save_to_file(&html_content, "nameservers.html") {
        Ok(_) => println!("HTML file generated successfully."),
        Err(e) => eprintln