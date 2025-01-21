use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

/// Parses all TLD files in the folder and finds domains associated with the name server.
pub fn find_domains_by_ns(folder_path: &str, name_server: &str) -> Result<Vec<String>, io::Error> {
    let mut domains = Vec::new();

    // Iterate over all files in the folder
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            // Process each TLD file
            if let Ok(file_domains) = parse_tld_file(&path, name_server) {
                domains.extend(file_domains);
            }
        }
    }

    Ok(domains)
}

/// Parses a single TLD file and extracts domains associated with the name server.
fn parse_tld_file(file_path: &Path, name_server: &str) -> Result<Vec<String>, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut domains = Vec::new();
    let mut current_domain = String::new();

    // Regex for matching domain and nserver lines
    let domain_regex = Regex::new(r"^domain:\s+([\w\.-]+)")?;
    let nserver_regex = Regex::new(r"^nserver:\s+([\w\.-]+)")?;

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = domain_regex.captures(&line) {
            current_domain = caps[1].to_string();
        } else if let Some(caps) = nserver_regex.captures(&line) {
            let ns = &caps[1];
           