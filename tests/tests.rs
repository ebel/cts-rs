use aws_sdk_controltower as controltower;
use controltower::error::SdkError;
use controltower::operation::list_landing_zones::{ListLandingZonesError, ListLandingZonesOutput};
use controltower::types::LandingZoneSummary;
use mockall::{ mock};

pub struct ControlTowerImpl {
    client: controltower::Client,
}

impl ControlTowerImpl {
    pub fn new(client: controltower::Client) -> Self {
        Self { client }
    }

    pub async fn list_landing_zones(&self) -> Result<ListLandingZonesOutput, SdkError<ListLandingZonesError>> {
        self.client.list_landing_zones().send().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_sdk_controltower::operation::list_landing_zones::ListLandingZonesOutput;

    mock! {
        pub ControlTowerImpl {
            async fn list_landing_zones(&self) -> Result<ListLandingZonesOutput, SdkError<ListLandingZonesError>>;
        }
    }

    #[tokio::test]
    async fn test_list_landing_zones() {
        let mut mock = MockControlTowerImpl::default();

        mock.expect_list_landing_zones()
            .return_once(|| {
                Ok(ListLandingZonesOutput::builder()
                    .landing_zones(
                        LandingZoneSummary::builder().arn("arn:aws:controltower:example:landingzone/1").build(),)
                    .build().expect("Failed to build ListLandingZonesOutput"))
            });

        let landing_zones_output = mock.list_landing_zones().await.unwrap();

        let landing_zone_arns: Vec<String> = landing_zones_output
            .landing_zones()
            .iter()
            .map(|lz| lz.arn().unwrap_or_default().to_string())
            .collect();
        assert_eq!(landing_zone_arns, vec![
            "arn:aws:controltower:example:landingzone/1".to_string()
        ]);
    }
}