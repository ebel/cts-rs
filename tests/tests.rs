use aws_sdk_controltower as controltower;
use aws_sdk_controltower::operation::list_enabled_controls::{ListEnabledControlsError, ListEnabledControlsOutput};
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

    pub async fn list_enabled_controls(&self) -> Result<ListEnabledControlsOutput, SdkError<ListEnabledControlsError>> {
        self.client.list_enabled_controls().send().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_sdk_controltower::operation::list_landing_zones::ListLandingZonesOutput;
    use aws_sdk_controltower::types::{DriftStatusSummary, EnabledControlSummary, EnablementStatusSummary};

    mock! {
        pub ControlTowerImpl {
            async fn list_landing_zones(&self) -> Result<ListLandingZonesOutput, SdkError<ListLandingZonesError>>;
            async fn list_enabled_controls(&self) -> Result<ListEnabledControlsOutput, SdkError<ListEnabledControlsError>>;
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
    #[tokio::test]
    async fn test_list_enabled_controls() {
        let mut mock = MockControlTowerImpl::default();

        mock.expect_list_enabled_controls()
            .return_once(|| {
                Ok(ListEnabledControlsOutput::builder()
                    .enabled_controls(
                        EnabledControlSummary::builder()
                            .control_identifier("arn:aws:controltower:us-west-2::control/test1")
                            .arn("arn:aws:controltower:us-west-2:account1:enabledcontrol/arn1")
                            .target_identifier("arn:aws:organizations::account1:ou/o-root/ou-1")
                            .status_summary(EnablementStatusSummary::builder().status("SUCCEEDED".to_string().parse().unwrap()).build())
                            .drift_status_summary(DriftStatusSummary::builder().drift_status("NOT_CHECKING".to_string().parse().unwrap()).build())
                            .build()
                    )
                    .enabled_controls(
                        EnabledControlSummary::builder()
                            .control_identifier("arn:aws:controltower:us-west-2::control/test2")
                            .arn("arn:aws:controltower:us-west-2:account2:enabledcontrol/arn2")
                            .target_identifier("arn:aws:organizations::account2:ou/o-root/ou-2")
                            .status_summary(EnablementStatusSummary::builder().status("SUCCEEDED".to_string().parse().unwrap()).build())
                            .drift_status_summary(DriftStatusSummary::builder().drift_status("NOT_CHECKING".to_string().parse().unwrap()).build())
                            .build()
                    )
                    .build().expect("Failed to build ListEnabledControlsOutput"))
            });

        let enabled_controls_output = mock.list_enabled_controls().await.unwrap();

        // Collecting and asserting all fields
        let enabled_control_ids: Vec<String> = enabled_controls_output
            .enabled_controls()
            .iter()
            .map(|control| control.control_identifier().unwrap_or_default().to_string())
            .collect();
        assert_eq!(enabled_control_ids, vec![
            "arn:aws:controltower:us-west-2::control/test1".to_string(),
            "arn:aws:controltower:us-west-2::control/test2".to_string()
        ]);

        let enabled_control_arns: Vec<String> = enabled_controls_output
            .enabled_controls()
            .iter()
            .map(|control| control.arn().unwrap_or_default().to_string())
            .collect();
        assert_eq!(enabled_control_arns, vec![
            "arn:aws:controltower:us-west-2:account1:enabledcontrol/arn1".to_string(),
            "arn:aws:controltower:us-west-2:account2:enabledcontrol/arn2".to_string()
        ]);

        let enabled_control_targets: Vec<String> = enabled_controls_output
            .enabled_controls()
            .iter()
            .map(|control| control.target_identifier().unwrap_or_default().to_string())
            .collect();
        assert_eq!(enabled_control_targets, vec![
            "arn:aws:organizations::account1:ou/o-root/ou-1".to_string(),
            "arn:aws:organizations::account2:ou/o-root/ou-2".to_string()
        ]);

        let enabled_control_statuses: Vec<String> = enabled_controls_output
            .enabled_controls()
            .iter()
            .map(|control| control.status_summary().unwrap().status().unwrap().to_string())
            .collect();
        assert_eq!(enabled_control_statuses, vec![
            "SUCCEEDED".to_string(),
            "SUCCEEDED".to_string()
        ]);

        let enabled_control_drift_statuses: Vec<String> = enabled_controls_output
            .enabled_controls()
            .iter()
            .map(|control| control.drift_status_summary().unwrap().drift_status().unwrap().to_string())
            .collect();
        assert_eq!(enabled_control_drift_statuses, vec![
            "NOT_CHECKING".to_string(),
            "NOT_CHECKING".to_string()
        ]);
    }
}