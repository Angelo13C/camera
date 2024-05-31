use core::ops::RangeInclusive;

use a13c_embedded::{peripherals::time::real_time::time::Time, utils::collections::list::List};

#[derive(Clone)]
pub enum EnableOnConditions<L: List<RangeInclusive<Time>>>
{
	Always,
	Never,
	TimeWindows
	{
		ranges: L,
	},
}

impl<L: List<RangeInclusive<Time>>> EnableOnConditions<L>
{
	pub fn should_be_enabled(&self, current_time: Time) -> bool
	{
		match self
		{
			EnableOnConditions::Always => return true,
			EnableOnConditions::Never => return false,
			EnableOnConditions::TimeWindows { ranges } =>
			{
				for i in 0..ranges.length()
				{
					if let Some(time_window) = ranges.get(i)
					{
						if time_window.contains(&current_time)
						{
							return true;
						}
					}
				}
			},
		}

		false
	}
}
