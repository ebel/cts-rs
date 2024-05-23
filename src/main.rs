mod control_tower;
mod organizations;

use clap::{Command, Arg,ArgAction};
//use std::io::{self, Write}; // For input/output operations
use anyhow::Result;

//Make calls to sdk DRY

extern crate cfonts;
use cfonts::{ say, Fonts, Colors, Options };

#[tokio::main]
async fn main() -> Result<()> {

    say(Options {
        text: String::from("cts-rs"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        ..Options::default()
    });

    let _matches = Command::new("CARGO_PKG_NAME")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("target_identifier")
            .help("Sets the target identifier (OU ARN)")
            .action(ArgAction::Set)
            .required(false))
        .get_matches();

    let control_tower_client =     control_tower::AwsControlTowerClient::new().await?;
    let organizations_client = organizations::AwsOrganizationsClient::new().await?;

    //let mut target_identifier = matches.get_one::<String>("target_identifier").map(String::from);

    // Check if target_identifier was not provided and prompt for it
    // if target_identifier.is_none() {
    //     println!("Please enter a target identifier that is not root (OU ARN):");
    //     let mut input = String::new();
    //     io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     target_identifier = Some(input.trim().to_string()); // Trim newline and other whitespace
    // }

    // Now you can unwrap because you've handled the case where it's missing. Verify..


    // let target_identifier = target_identifier.unwrap();
    // println!("Using target identifier (OU ARN): {}", target_identifier);

    // Call list_roots and get the parent_id
    if let Some(parent_id) = organizations_client.list_roots().await? {
        // Call list_organizational_units_for_parent with the obtained parent_id
        organizations_client.list_organizational_units_for_parent(&parent_id).await?;

        //control_tower_client.list_enabled_controls().await?;
    }


    //control_tower_client.list_landing_zones().await?;



    //control_tower_client.list_baselines().await?;

    //let rootou = "r-zrjd";
    //organizations_client.describe_organization().await?;
    //organizations_client.list_roots().await?;

    // Call list_roots and get the parent_id
    if let Some(parent_id) = organizations_client.list_roots().await? {
        // Call list_organizational_units_for_parent with the obtained parent_id
        organizations_client.list_organizational_units_for_parent(&parent_id).await?;
    }

    Ok(())
}

//TODO: Remove command line prompt for child OU.
//TODO: Use root ou method to get root ou id and child and use info for other methods.
//TODO: ADD OU name to each enabled control group
//TODO: Format output better
//TODO: Command line arguements for:
//      * Get info for root (id, name, arn)
//      * Get info for each child OU (id, name, arn)
//      * Print for both root and child (id, name, arn)
//
//TODO: Params required. IF none give required message to rerun/etc.
//
//
//
//
