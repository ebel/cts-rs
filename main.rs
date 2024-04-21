use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_controltower as controltower;

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
        println!("No landing zones returned");
    } else {
        for landing_zone in response.landing_zones {
            println!("Control Tower Landing Zone: {:?}", landing_zone);
        }
    }
    // Lookup org name
    let target_identifier = "xxx";
    let response = client.list_enabled_controls().target_identifier(target_identifier).send().await?;

    // Directly work with the response since it's not a Result type
    if response.enabled_controls.is_empty() {
        println!("No controls returned");
    } else {
        for control in response.enabled_controls {
            println!("Control Tower Control is: {:?}", control);
        }
    }

    Ok(())
}
