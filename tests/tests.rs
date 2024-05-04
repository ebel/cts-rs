// use aws_sdk_controltower as controltower;
// #[allow(unused_imports)]
// use mockall::{automock, predicate::*};
//
// use controltower::error::SdkError;
// use controltower::operation::list_landing_zones::{ListLandingZonesError, ListLandingZonesOutput};
// use controltower::types::{LandingZoneSummary};
//
//
// #[cfg(test)]
// pub use MockControlTowerImpl as S3;
// #[cfg(not(test))]
// pub use ControlTowerImpl as S3;
//
// #[allow(dead_code)]
// pub struct ControlTowerImpl {
//     inner: controltower::Client,
// }
//
// #[cfg_attr(test, automock)]
// impl ControlTowerImpl {
//     #[allow(dead_code)]
//     pub fn new(inner: controltower::Client) -> Self {
//         Self { inner }
//     }
//
//     #[allow(dead_code)]
//     pub async fn list_landing_zones(
//         &self
//     ) -> Result<ListLandingZonesOutput, SdkError<ListLandingZonesError>> {
//         self.inner
//             .list_landing_zones()
//             .send()
//             .await
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_list_landing_zone() {
//         let mut mock = MockControlTowerImpl::default();
//
//         // Define the expected output using builder methods
//         let arn_string = "arn:aws:controltower:example-zone1".to_string();
//         let landing_zone = LandingZoneSummary::builder()
//             .arn(arn_string)  // Directly pass a String
//             .build();  // Assumes building cannot fail
//
//         let expected_output = ListLandingZonesOutput::builder()
//             .landing_zones(landing_zone)
//             .build();
//
// // Setup expectations
//         mock.expect_list_landing_zones()
//             .times(1)
//             .returning(move || {
//                 // Define the output within the returning closure
//                 let landing_zone_summary = LandingZoneSummary::builder()
//                     .arn("arn:aws:controltower:example-zone1".to_string())
//                     .build();  // Assumes building cannot fail
//
//                 let expected_output = ListLandingZonesOutput::builder()
//                     .landing_zones(vec![landing_zone_summary])  // Correctly using a Vec<LandingZoneSummary>
//                     .next_token(Some("next_page_token".to_string()))  // Optionally include a next token
//                     .build();  // Also assumes building cannot fail
//
//                 Ok(expected_output)  // Return Ok wrapping the expected output
//             });
//
//
//         // Call the method
//         let result = mock.list_landing_zones().await;
//
//         // Assert the expected result
//         assert!(result.is_ok());
//         let output = result.unwrap();
//         assert_eq!(output.landing_zones.len(), 1);
//         assert_eq!(output.landing_zones[0].name.as_deref(), Some("Zone1"));
//     }
// }
