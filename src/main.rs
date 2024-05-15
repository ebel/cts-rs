mod control_tower;
mod organizations;

use clap::{Command, Arg,ArgAction};
use std::io::{self, Write}; // For input/output operations

//Make calls to sdk DRY

extern crate cfonts;
use cfonts::{ say, Fonts, Colors, Options };

#[tokio::main]
async fn main() -> Result<(), aws_sdk_controltower::Error> {

    say(Options {
        text: String::from("cts-rs"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        ..Options::default()
    });

    let matches = Command::new("CARGO_PKG_NAME")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("target_identifier")
            .help("Sets the target identifier (OU ARN)")
            .action(ArgAction::Set)
            .required(false))
        .get_matches();

    let mut target_identifier = matches.get_one::<String>("target_identifier").map(String::from);

    // Check if target_identifier was not provided and prompt for it
    if target_identifier.is_none() {
        println!("Please enter a target identifier that is not root (OU ARN):");
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        io::stdin().read_line(&mut input).expect("Failed to read line");
        target_identifier = Some(input.trim().to_string()); // Trim newline and other whitespace
    }

    // Now you can unwrap because you've handled the case where it's missing. Verify..
    let target_identifier = target_identifier.unwrap();
    println!("Using target identifier (OU ARN): {}", target_identifier);

    let control_tower_client =     control_tower::AwsControlTowerClient::new().await?;

    control_tower_client.list_landing_zones().await?;

    control_tower_client.list_enabled_controls(target_identifier).await?;

    control_tower_client.list_baselines().await?;

    Ok(())
}
