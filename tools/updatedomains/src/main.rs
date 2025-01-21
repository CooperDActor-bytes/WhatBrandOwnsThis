mod parser;

use clap::{Arg, Command};
use parser::find_domains_by_ns;
use std::fs;

fn main() {
    let matches = Command::new("Zone File Parser")
        .version("1.0")
        .author("cooper <cooper@salty.cool>")
        .about("Finds all domains associated with specified name servers in plain-text zone files")
        .arg(
            Arg::new("folder")
                .about("Path to the folder containing TLD files")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("nameservers_file")
                .about("Path to the text file containing name servers to search for")
                .required(true)
                .index(2),
        )
        .get_matches();

    let folder_path = matches.value_of("folder").unwrap();
    let nameservers_file = matches.value_of("nameservers_file").unwrap();

    match read_nameservers_from_file(nameservers_file) {
        Ok(name_servers) => {
            match find_domains_by_ns(folder_path, &name_servers) {
                Ok(domains) => {
                    for (name_server, domain_list) in domains {
                        println!("\nDomains using '{}':", name_server);
                        for domain in domain_list {
                            println!("- {}", domain);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading name servers from file: {}", err);
        }
    }
}

/// Reads name servers from a given file, each on a new line.
fn read_nameservers_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let name_servers = content
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    Ok(name_servers)
}