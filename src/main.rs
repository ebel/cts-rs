use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_controltower as controltower;
use colored::*; // Import the colored crate


//Make calls to sdk DRY

#[tokio::main]
async fn main() -> Result<(), controltower::Error> {
    // Setup AWS region
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");

    // Setup AWS configuration
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    // Create a Control Tower client
    let client = aws_sdk_controltower::Client::new(&config);

    // Retrieve and display the current status of AWS Control Tower
    let response = client.list_landing_zones().send().await?;


    // Print out the status
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
    }
    // Lookup org name
    let target_identifier = "";
    let response = client.list_enabled_controls().target_identifier(target_identifier).send().await?;

    // Directly work with the response since it's not a Result type
    if response.enabled_controls.is_empty() {
        println!("No controls returned");
    } else {

        use colored::*; // Import the colored crate at the top of your Rust file

        for control in &response.enabled_controls {
            println!("{}", "Control Tower Control is:".blue());

            // Use to_string directly on the optional references
            let control_identifier_label = "Control Identifier:".blue().to_string();
            let control_identifier = control.control_identifier.as_ref().map(|s| s.to_string()).unwrap_or("None".to_string());
            println!("{} {}", control_identifier_label, control_identifier);

            let arn_label = "ARN:".blue().to_string();
            let arn = control.arn.as_ref().map(|s| s.to_string()).unwrap_or("None".to_string());
            println!("{} {}", arn_label, arn);

            let target_identifier_label = "Target Identifier:".blue().to_string();
            let target_identifier = control.target_identifier.as_ref().map(|s| s.to_string()).unwrap_or("None".to_string());
            println!("{} {}", target_identifier_label, target_identifier);

            match &control.status_summary {
                Some(status_summary) => {
                    let status = status_summary.status.as_ref().map(|s| s.to_string()).unwrap_or("None".to_string());
                    println!("  Status: {}", status);
                },
                None => println!("  Status: None"),
            }

            match &control.drift_status_summary {
                Some(drift_status_summary) => {
                    let drift_status = drift_status_summary.drift_status.as_ref().map(|s| s.to_string()).unwrap_or("None".to_string());
                    println!("  Drift Status: {}", drift_status);
                },
                None => println!("  Drift Status: None"),
            }

            println!(); // Add a blank line for better separation between entries
        }


    }

    Ok(())
}
