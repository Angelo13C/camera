mod enable_on;

use core::ops::RangeInclusive;

use a13c_embedded::{
	drivers::input_pin::DelayedInputPin,
	peripherals::time::real_time::time::{Date, Duration, Time},
	utils::collections::list::List,
};
use embedded_hal::digital::InputPin;
pub use enable_on::*;

/// Decides if the camera should capture images and also if it should store the image in the storage device.
pub struct ImageTrigger<P: InputPin, L: List<RangeInclusive<Time>>>
{
	pir_sensor: DelayedInputPin<P, Duration>,

	date_and_time_of_last_tick: Option<(Date, Time)>,
	trigger_date_and_time: Option<(Date, Time)>,
	trigger_duration: Duration,

	is_enabled: bool,
	enable_on: EnableOnConditions<L>,
}

impl<P: InputPin, L: List<RangeInclusive<Time>>> ImageTrigger<P, L>
{
	pub fn new(pir_sensor: P, enable_on: EnableOnConditions<L>, trigger_duration: core::time::Duration) -> Self
	{
		Self {
			pir_sensor: DelayedInputPin::new(pir_sensor, false, Duration::seconds(60)),

			date_and_time_of_last_tick: None,
			trigger_duration: trigger_duration.try_into().unwrap(),
			trigger_date_and_time: None,

			is_enabled: false,
			enable_on,
		}
	}

	pub fn tick(&mut self, current_date_and_time: Option<(Date, Time)>) -> Result<(), P::Error>
	{
		if let Some((current_date, current_time)) = current_date_and_time
		{
			self.is_enabled = self.enable_on.should_be_enabled(current_time);

			if self.are_date_and_time_of_last_tick_valid(current_date, current_time)
			{
				self.pir_sensor
					.tick(self.duration_since_last_tick(current_date, current_time));
			}

			if self.is_enabled
			{
				if self.pir_sensor.is_high()?
				{
					self.trigger_date_and_time = Some((current_date, current_time));
				}
			}
		}
		self.date_and_time_of_last_tick = current_date_and_time;

		Ok(())
	}

	/// Check the struct's documentation.
	pub fn needs_to_capture_image(&self) -> bool
	{
		self.is_enabled
	}

	/// Check the struct's documentation.
	pub fn needs_to_store_image(&self) -> bool
	{
		return false;
		if let Some((trigger_date, trigger_time)) = self.trigger_date_and_time
		{
			let duration_since_trigger = -self.duration_since_last_tick(trigger_date, trigger_time);
			duration_since_trigger <= self.trigger_duration
		}
		else
		{
			false
		}
	}

	fn are_date_and_time_of_last_tick_valid(&self, current_date: Date, current_time: Time) -> bool
	{
		self.duration_since_last_tick(current_date, current_time) < Duration::days(365)
	}

	fn duration_since_last_tick(&self, date: Date, time: Time) -> Duration
	{
		if let Some(date_and_time_of_last_tick) = self.date_and_time_of_last_tick
		{
			(date - date_and_time_of_last_tick.0) + (time - date_and_time_of_last_tick.1)
		}
		else
		{
			Duration::ZERO
		}
	}
}
