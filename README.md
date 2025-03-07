
<a name="readme-top"></a>

[![Unit Tests](https://github.com/PeterMcQuaid/keccak_cli_tool/actions/workflows/rust.yml/badge.svg)](https://github.com/PeterMcQuaid/keccak_cli_tool/actions/workflows/rust.yml) [![Rust Version](https://img.shields.io/badge/rust-1.85.0+-blue.svg)](https://www.rust-lang.org) [![License](https://img.shields.io/badge/license-MIT-green)](LICENSE) 


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/PeterMcQuaid/keccak_cli_tool/images">
    <img src="https://raw.githubusercontent.com/PeterMcQuaid/keccak_cli_tool/main/images/logo.jpg" alt="Logo" width=500>
  </a>

  <h3 align="center">Keccak CLI Tool</h3>
  <p align="center">
    <br />
    <a href="https://github.com/PeterMcQuaid/keccak_cli_tool#installation"><strong>Setup & Installation »</strong></a>
    <br />
    <br />
    <a href="https://github.com/PeterMcQuaid/keccak_cli_tool#contributions">Contribute to Keccak CLI Tool</a>
    ·
    <a href="https://github.com/PeterMcQuaid/keccak_cli_tool/issues">Report Bug</a>
    ·
    <a href="https://github.com/PeterMcQuaid/keccak_cli_tool/issues">Request Feature</a>
  </p>
</div>

## Description

Simple Rust CLI tool for Keccak256 hashing strings and file contents

## Installation

```bash
git clone https://github.com/PeterMcQuaid/keccak_cli_tool.git
cd keccak_cli_tool
cargo build
```
    
## Testing
    
1. Run unit-tests in root directory 

    ```
    cargo test
    ```

## Usage

1. Test simple string and contents of `test.txt`:

    ```
    cargo run -- "hello" "test.txt"
    ```
  
## Contributions

Pull requests are welcome! Please ensure that any changes or additions you make are well-documented and covered by test cases.

For any bugs or issues, please open an [issue](https://github.com/PeterMcQuaid/keccak_cli_tool/issues).

  
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details