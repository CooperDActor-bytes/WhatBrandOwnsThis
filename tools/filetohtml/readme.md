# Nameservers to HTML Generator

This is a simple Rust tool that takes a list of nameservers (in JSON format) and generates an HTML file displaying the nameservers in an unordered list. This can be useful for quickly converting a list of nameservers into a human-readable HTML format for documentation or sharing.

## Requirements

- [Rust](https://www.rust-lang.org/learn/get-started) installed on your system. You can install it using `rustup` by following the instructions on the official Rust website.

## Installation

### 1. Clone the Repository (if applicable)

If you are cloning this project from a Git repository:

```sh
git clone https://your-repository-url.git
cd nameservers_to_html

2. Create the Rust Project (if starting from scratch)

If you are starting fresh, create a new Rust project with:

cargo new nameservers_to_html
cd nameservers_to_html

Then, copy the following dependencies into your Cargo.toml file:

[dependencies]
serde = { version = “1.0”, features = [“derive”] }
serde_json = “1.0”
tokio = { version = “1”, features = [“full”] }

3. Replace src/main.rs

Replace the content of src/main.rs with the code provided in the main section of the README.

// Add the Rust code here from the main section of the README

Usage

1. Modify JSON Input

You can customize the json_input variable in the main.rs file with your desired nameservers. The program expects a JSON format like this:

{
    “nameservers”: [
        “ns1.example.com”,
        “ns2.example.com”,
        “ns3.example.com”
    ]
}

2. Run the Program

Once you’ve set the nameservers, you can run the program using the following command:

cargo run

This will generate an HTML file (nameservers.html) with the nameservers listed in an unordered list.

3. View the HTML Output

After running the program, an nameservers.html file will be generated in the project directory. Open the file in a browser to see the list of nameservers.

Example of HTML output:

<!DOCTYPE html>
<html lang=“en”>
<head>
    <meta charset=“UTF-8”>
    <meta name=“viewport” content=“width=device-width, initial-scale=1.0”>
    <title>Nameservers</title>
</head>
<body>
    <h1>Nameservers</h1>
    <ul>
        <li>ns1.example.com</li>
        <li>ns2.example.com</li>
        <li>ns3.example.com</li>
    </ul>
</body>
</html>

—

