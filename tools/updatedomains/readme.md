# Zone File Parser

**Zone File Parser** is a Rust-based CLI tool that processes TLD zone files to extract all domains associated with one or more specified name servers. This tool is designed for analyzing large datasets of plain-text zone files and finding relevant domain-to-name-server mappings.

## Features

- Scans all TLD files in a specified folder.
- Extracts domains associated with one or more specified name servers.
- Supports plain-text zone file formats.
- Lightweight and fast.

—

## Installation

### Prerequisites

- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **TLD Files**: Ensure zone files are in plain-text format and located in a folder (e.g., `zone/`).

### Clone and Build

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/zone_file_parser.git
   cd zone_file_parser

	2.	Build the project:

cargo build —release


	3.	The compiled binary will be available at:

./target/release/zone_file_parser

Usage

Run the parser with the following syntax:

zone_file_parser <path_to_folder> <nameservers_file>

Where:
	•	<path_to_folder>: The path to the folder containing TLD zone files.
	•	<nameservers_file>: A text file that contains one name server per line.

Example

Given the following TLD file in zone/com:

domain:       example.com
nserver:      ns1.example.com
nserver:      ns2.example.com

domain:       anotherdomain.net
nserver:      ns1.example.com
nserver:      ns3.example.com

domain:       example.org
nserver:      ns2.example.com
nserver:      ns4.example.com

And a text file nameservers.txt that contains:

ns1.example.com
ns2.example.com

Run the tool like this:

./target/release/zone_file_parser zone/ nameservers.txt

Output:

Domains using ‘ns1.example.com’:
- example.com
- anotherdomain.net

Domains using ‘ns2.example.com’:
- example.com
- example.org

Options
	•	<path_to_folder>: Path to the folder containing TLD zone files.
	•	<nameservers_file>: A text file with one name server per line. Example:

ns1.example.com
ns2.example.com

Folder and File Format

The tool expects zone files in a folder with the following structure:

zone/
├── com
├── net
├── org

Each file is plain text and formatted as follows:

domain:       example.com
nserver:      ns1.example.com
nserver:      ns2.example.com

Development

Running Tests

Run the following command to test the project:

cargo test

Formatting

Ensure your code is properly formatted:

cargo fmt

Future Improvements
	•	Add support for output formats (e.g., JSON, CSV).
	•	Include support for parallel processing for large datasets.
	•	Validate and handle malformed files gracefully.

Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

License

This project is licensed under the MIT License.

Acknowledgments
	•	Rust Programming Language
	•	IANA

—

### Key Points

- **Name Servers File**: You now provide a text file containing a list of name servers (one per line).
- **Command Usage**: The user passes the folder path and the name servers file path as arguments.
- **File Format**: Each line in the text file should contain one name server, e.g., `ns1.example.com`.

This updated setup provides more flexibility and makes it easier to manage multiple name servers without having to modify the command line every time.