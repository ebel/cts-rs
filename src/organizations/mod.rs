use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_organizations::{Client as OrganizationsClient};
use anyhow::Result;

use colored::*;

pub struct AwsOrganizationsClient {
    client: OrganizationsClient,
}
impl AwsOrganizationsClient {
    // Constructs a new `AwsOrganizationsClient`.
    pub async fn new() -> Result<Self> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let client = OrganizationsClient::new(&config);
        Ok(AwsOrganizationsClient { client })
    }

    pub async fn list_roots(&self) -> Result<Option<String>> {
         let response = self.client.list_roots().send().await?;

        if response.roots.is_none() {
            println!("No root OU returned");

        } else {
            for ou in response.roots() {
                let parent_id = ou.id().unwrap_or("None").to_string();
                println!("{} {}", "Root OU id is:".blue().to_string(), ou.id().unwrap_or("None"));
                println!("");
                return Ok(Some(parent_id));
            }
        }
        Ok(None)

    }
    pub async fn list_organizational_units_for_parent(&self, parent_id: &str, ) -> Result<()> {
        let response = self.client.list_organizational_units_for_parent().parent_id(parent_id).send().await?;

        if response.organizational_units.is_none() {
            println!("No ous returned");
        } else {
            for ou in response.organizational_units() {
                println!("{} {}","OU id:".blue().to_string(), ou.id().unwrap_or("None"));
                println!("{} {}","OU name:".blue().to_string(), ou.name().unwrap_or("None"));
                println!("{} {}","OU arn:".blue().to_string(), ou.arn().unwrap_or("None"));
                println!("");
            }
        }
        Ok(())
        // pub async fn describe_organizational_unit(&self) -> Result<()>{
        //     let response = self.client.describe_organizational_unit().send().await?;
        //
        //     if let Some(_organization) = response.organization() {
        //         for ou in &response.organization {
        //             println!("OU id is: {}", ou.id().unwrap_or("None"));
        //         }
        //     } else {
        //         println!("No organization found.");
        //     }
        //
        //     Ok(())
        // }
    }
}