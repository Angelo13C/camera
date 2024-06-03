use core::{ops::RangeInclusive, time::Duration};

use a13c_embedded::peripherals::time::real_time::time::Time;
use firmware_core::{
	configuration::customization::Customization as CustomizationTrait, features::trigger::EnableOnConditions,
};
pub struct Customization;

impl CustomizationTrait for Customization
{
	type EnableOnConditionsList = Vec<RangeInclusive<Time>>;

	fn enable_image_trigger_on(&self) -> EnableOnConditions<Self::EnableOnConditionsList>
	{
		EnableOnConditions::TimeWindows {
			ranges: vec![
				Time::from_hms(0, 0, 0).unwrap()..=Time::from_hms(23, 59, 59).unwrap(),
			],
		}
	}

	fn trigger_duration(&self) -> Duration
	{
		Duration::from_secs(3 * 60 * 60)
	}
}
