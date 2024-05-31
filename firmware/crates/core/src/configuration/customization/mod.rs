use core::{ops::RangeInclusive, time::Duration};

use a13c_embedded::{peripherals::time::real_time::time::Time, utils::collections::list::List};

use crate::features::trigger::EnableOnConditions;

pub trait Customization
{
	type EnableOnConditionsList: List<RangeInclusive<Time>>;

	fn enable_image_trigger_on(&self) -> EnableOnConditions<Self::EnableOnConditionsList>;
	fn trigger_duration(&self) -> Duration;
}
