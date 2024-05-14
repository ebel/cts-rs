# cts-rs

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ebel/cts-rs/rust.yml)
![Crates.io Version](https://img.shields.io/crates/v/cts-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/cts-rs)

<picture>
 <source media="(prefers-color-scheme: dark)" srcset="https://github.com/ebel/cts-rs/blob/main/repo_pic.png">
 <source media="(prefers-color-scheme: light)" srcset="https://github.com/ebel/cts-rs/blob/main/repo_pic.png">
 <img alt="YOUR-ALT-TEXT" src="https://github.com/ebel/cts-rs/blob/main/repo_pic.png">
</picture>


A command line utility using the AWS Rust SDK to get all the available information for your AWS Control Tower setup.


* Auth via [aws creds](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html) before using.
* Use an organzation ARN that is not the root OU.

## Usage
* Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* `cargo install cts-rs --version "<replace with latest version here>"`

## TODO:
- [ ] Support all read operations of AWS Control Tower Rust SDK
- [ ] Table, json or text output
- [ ] List Accounts enrolled in AWS Control Tower
- [ ] List OUs enrolled in AWS Control Tower
- [ ] Tests all functions
- [ ] Help menu
- [ ] Paging results (Less/More)
- [ ] Command line input
  - [X] Initial OU prompt at startup
- [ ] Output color formatting
- [ ] Build, Test, Release GHA workflows
- [X] Signed commits setup
- [ ] TBD

