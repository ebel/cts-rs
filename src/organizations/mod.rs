use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_controltower::{Client as OrganizationsClient, Error as OrganizationsError};


pub struct AwsOrganizationsClient {
    client: OrganizationsClient,
}
impl AwsOrganizationsClient {
    // Constructs a new `AwsOrganizationsClient`.
    pub async fn new() -> Result<Self, aws_sdk_controltower::Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let client = OrganizationsClient::new(&config);
        Ok(AwsOrganizationsClient { client })
    }
}

