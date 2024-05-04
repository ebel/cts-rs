use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
//use aws_sdk_controltower as controltower;
use aws_sdk_controltower::operation::list_baselines::ListBaselinesOutput; // Correct import for ListBaselinesOutput
use aws_sdk_controltower::{Client as ControlTowerClient, Error as ControlTowerError};

use colored::*; // Import the colored crate
use clap::{Command};

//Make calls to sdk DRY

extern crate cfonts;

use cfonts::{ say, Fonts, Colors, Options };


pub struct AwsControlTowerClient {
    client: ControlTowerClient,
}

impl AwsControlTowerClient {
    // Constructs a new `AwsControlTowerClient`.
    pub async fn new() -> Result<Self, aws_sdk_controltower::Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let client = ControlTowerClient::new(&config);
        Ok(AwsControlTowerClient { client })
    }

    // Method to list landing zones
    pub async fn list_landing_zones(&self) -> Result<(), aws_sdk_controltower::Error> {
        let response = self.client.list_landing_zones().send().await?;

        if response.landing_zones.is_empty() {
            println!("{}", "No landing zones returned".red());
        } else {
            for landing_zone in response.landing_zones {
                if let Some(arn) = &landing_zone.arn {
                    println!("{} {}", "Control Tower Landing Zone ARN is:".blue(), arn);
                } else {
                    println!("{}", "No ARN available for this landing zone".yellow());
                }
            }
            println!();
        }
        Ok(())
    }

    // Method to list enabled controls for a given target identifier
    pub async fn list_enabled_controls(&self, target_identifier: &str) -> Result<(), aws_sdk_controltower::Error> {
        let response = self.client.list_enabled_controls().target_identifier(target_identifier).send().await?;

        println!("{}", "Control Tower Control is:".blue());

        if response.enabled_controls.is_empty() {
            println!("No controls returned");
        } else {
            for control in &response.enabled_controls {
                println!("{} {}", "Control Identifier:".blue().to_string(), control.control_identifier.as_ref().unwrap_or(&"None".to_string()));
                println!("{} {}", "ARN:".blue().to_string(), control.arn.as_ref().unwrap_or(&"None".to_string()));
                println!("{} {}", "Target Identifier:".blue().to_string(), control.target_identifier.as_ref().unwrap_or(&"None".to_string()));

                let status = control.status_summary.as_ref()
                    .and_then(|summary| summary.status.as_ref())
                    .map(|status| status.to_string())
                    .unwrap_or("None".to_string());
                println!("  Status: {}", status);

                let drift_status = control.drift_status_summary.as_ref()
                    .and_then(|summary| summary.drift_status.as_ref())
                    .map(|status| status.to_string())
                    .unwrap_or("None".to_string());
                println!("  Drift Status: {}", drift_status);

                println!(); // Add a blank line for better separation between entries
            }
        }
        Ok(())
    }

    // Lists the baselines in AWS Control Tower
    pub async fn list_baselines(&self) -> Result<ListBaselinesOutput, ControlTowerError> {
        match self.client.list_baselines().send().await {
            Ok(response) => {
                println!("Baselines:");
                for baseline in &response.baselines {
                    let arn = &baseline.arn;
                    let name = &baseline.name;
                    let _description = &baseline.description;
                    println!("ARN: {}", arn);
                    println!("Name: {}", name);
                    let _description = baseline.description.as_deref().unwrap_or("No description available");                    println!(); // Adds a blank line for better separation
                }

                if let Some(next_token) = &response.next_token {
                    println!("Next token for pagination: {}", next_token);
                } else {
                    println!("No more pages.");
                }

                Ok(response)
            },
            Err(e) => {
                println!("Error fetching baselines: {}", e);
                Err(ControlTowerError::from(e))
            }
        }
    }
    // pub async fn list_enabled_baselines(&self) -> Result<ListEnabledBaselinesOutput, ControlTowerError> {
    //     match self.client.list_enabled_baselines().send().await {
    //         Ok(response) => {
    //             println!("Enabled Baselines:");
    //             for baseline in &response.baselines {
    //                 let arn = &baseline.arn;
    //                 let name = &baseline.name;
    //                 let _description = &baseline.description;
    //                 println!("ARN: {}", arn);
    //                 println!("Name: {}", name);
    //                 let _description = baseline.description.as_deref().unwrap_or("No description available");                    println!(); // Adds a blank line for better separation
    //             }
    //
    //             if let Some(next_token) = &response.next_token {
    //                 println!("Next token for pagination: {}", next_token);
    //             } else {
    //                 println!("No more pages.");
    //             }
    //
    //             Ok(response)
    //         },
    //         Err(e) => {
    //             println!("Error fetching baselines: {}", e);
    //             Err(ControlTowerError::from(e))
    //         }
    //     }
    // }
}

#[tokio::main]
async fn main() -> Result<(), aws_sdk_controltower::Error> {

    say(Options {
        text: String::from("cts-rs"),
        font: Fonts::FontBlock,
        colors: vec![Colors::System],
        ..Options::default()
    });

    let app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION")); // No `takes_value()` needed

    let _matches = app.get_matches();

    let control_tower_client = AwsControlTowerClient::new().await?;
    control_tower_client.list_landing_zones().await?;

    let target_identifier = "";
    control_tower_client.list_enabled_controls(target_identifier).await?;

    control_tower_client.list_baselines().await?;

    Ok(())
}
